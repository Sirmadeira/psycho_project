//! Plugin responsible for customizing the player character in rtt and the final result shall be used and replicated when enter ingame state
use crate::client::load_assets::CharCollection;
use crate::client::MyAppState;
use crate::shared::protocol::player_structs::*;
use bevy::animation::AnimationTarget;
use bevy::prelude::*;
use bevy::utils::HashMap;
use lightyear::client::events::MessageEvent;
use lightyear::connection::id::ClientId;
use lightyear::prelude::client::Predicted;
use lightyear::shared::replication::components::Controlled;
use std::collections::VecDeque;

pub struct CustomizeCharPlugin;

impl Plugin for CustomizeCharPlugin {
    fn build(&self, app: &mut App) {
        //Events
        app.add_event::<TranferAnim>();
        app.add_event::<ResetAnimation>();

        // Player debugging
        app.register_type::<PlayerVisuals>();
        app.register_type::<SavePlayerBundleMap>();

        // Starting up base resource
        app.init_resource::<BodyPartMap>();
        app.init_resource::<SkeletonMap>();

        //Debugging
        app.register_type::<BodyPartMap>();
        app.register_type::<SkeletonMap>();

        // Creates player - RC because he should only run in game, not before and such
        app.add_systems(
            Update,
            formulates_players.run_if(in_state(MyAppState::Game)),
        );

        // System to customize character correctly
        app.add_systems(Update, customizes_character);

        // Does the anim transfer - I know last here is weird but here is the thing because of the way child entities spawn in bevy
        // We need to wait a good while before running this guy
        app.add_systems(Last, transfer_essential_components);

        app.add_systems(Update, reset_animation);
    }
}

/// Resource that tell me which assets had their animation targets transfered
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct BodyPartMap(pub HashMap<(ClientId, String), Entity>);

/// If you pass the client id it will tell you exactly who are all child entities of it
impl BodyPartMap {
    // Function to find all entities associated with a given ClientId
    pub fn find_entities_by_client_id(&self, client_id: &ClientId) -> Vec<Entity> {
        self.0
            .iter()
            .filter_map(
                |((id, _part_name), entity)| {
                    if id == client_id {
                        Some(*entity)
                    } else {
                        None
                    }
                },
            )
            .collect()
    }
}

/// Tell me who is the current skeleton of that player
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
struct SkeletonMap(HashMap<ClientId, Entity>);

/// Tell me when to transfer the anim of a certain player
#[derive(Event, Reflect)]
struct TranferAnim(ClientId);

/// Resets animations when i finish the transition
#[derive(Event, Reflect)]
struct ResetAnimation(ClientId);

/// A simple component that tells me if it already transfered the animation targets
#[derive(Component)]
struct HasTarget;

/// A simple component that tell me if side player has it is visuals spawned
#[derive(Component)]
struct HasVisuals;

/// Helper function spawns a series of scenes acording to the given batch of visuals being passed
fn spawn_scene(
    visual: &str,
    client_collection: &Res<CharCollection>,
    gltfs: &Res<Assets<Gltf>>,
    commands: &mut Commands,
) -> Entity {
    // Retrieve the GLTF handle from the collection, error if not found
    let gltf = client_collection
        .gltf_files
        .get(visual)
        .expect(&format!("Couldn't find GLTF file path for: {}", visual));

    // Retrieve the loaded GLTF and its first scene, expect it to exist
    let loaded_gltf = gltfs
        .get(gltf)
        .expect("To find gltf handle in loaded gltfs");
    let visual_scene = loaded_gltf.scenes[0].clone();

    let scene = SceneBundle {
        scene: visual_scene,
        transform: Transform::from_translation(Vec3::new(0.0, -0.75, 0.0)),
        ..default()
    };

    // info!("Spawning visual {} scene", visual);

    // Spawn and return the appropriate entity

    let id = commands.spawn(scene).id();
    id
}

/// Helper Collects a lot of subchild bones
pub fn collect_bones(
    children_entities: &Query<&Children>,
    names: &Query<&Name>,
    root_bone: &Entity,
    collected: &mut HashMap<String, Entity>,
) {
    if let Ok(name) = names.get(*root_bone) {
        collected.insert(format!("{}", name), *root_bone);

        if let Ok(children) = children_entities.get(*root_bone) {
            for child in children {
                collect_bones(children_entities, names, child, collected)
            }
        }
    }
}

/// Helper Finds a bone with a certain name
fn find_child_with_name_containing(
    children_entities: &Query<&Children>,
    names: &Query<&Name>,
    entity: &Entity,
    name_to_match: &str,
) -> Option<Entity> {
    let mut queue = VecDeque::new();
    queue.push_back(entity);

    while let Some(curr_entity) = queue.pop_front() {
        let name_result = names.get(*curr_entity);
        if let Ok(name) = name_result {
            if format!("{}", name).contains(name_to_match) {
                // found the named entity
                return Some(*curr_entity);
            }
        }

        let children_result = children_entities.get(*curr_entity);
        if let Ok(children) = children_result {
            for child in children {
                queue.push_back(child)
            }
        }
    }

    return None;
}

/// Spawns visuals scenes and parents them to predicted player
fn formulates_players(
    main_player: Query<
        (
            Entity,
            &PlayerId,
            &PlayerVisuals,
            Has<HasVisuals>,
            Has<Controlled>,
        ),
        (Added<Predicted>, With<MarkerPlayer>),
    >,
    gltfs: Res<Assets<Gltf>>,
    client_collection: Res<CharCollection>,
    mut body_part_map: ResMut<BodyPartMap>,
    mut skeleton_map: ResMut<SkeletonMap>,
    mut transfer_anim: EventWriter<TranferAnim>,
    mut commands: Commands,
) {
    for (entity, player_id, player_visuals, has_visual, is_controlled) in main_player.iter() {
        info_once!("Found player {}", entity);
        if !has_visual {
            let client_id = player_id.0;

            info!(
                "Inserting additonal info  component in clientplayer {}",
                client_id
            );

            if is_controlled {
                info!("Inserting input map into main player and adding clientsided components");
                commands
                    .entity(entity)
                    .insert(InheritedVisibility::default())
                    .insert(GlobalTransform::default())
                    .insert(HasVisuals)
                    .insert(PlayerAction::default_input_map());
            } else {
                info!("Insert additional components in side player");
                commands
                    .entity(entity)
                    .insert(InheritedVisibility::default())
                    .insert(GlobalTransform::default())
                    .insert(HasVisuals);
            }

            for file_path in player_visuals.iter_visuals() {
                if file_path.contains("skeleton") {
                    info!("Found side player skeleton");
                    let visual_scene =
                        spawn_scene(&file_path, &client_collection, &gltfs, &mut commands);
                    commands.entity(visual_scene).set_parent(entity);

                    info!("Inserting skeleton into map");
                    skeleton_map.0.insert(client_id, visual_scene);
                } else {
                    let visual_scene =
                        spawn_scene(&file_path, &client_collection, &gltfs, &mut commands);

                    commands.entity(visual_scene).set_parent(entity);

                    body_part_map
                        .0
                        .insert((client_id, file_path.to_string()), visual_scene);
                }
            }
            info!(
                "Telling client {} to transfer animation targets according to his skeleton",
                client_id
            );
            transfer_anim.send(TranferAnim(client_id));
        } else {
            info_once!("This player already has visuals {}", entity)
        }
    }
}

/// Customizes character after a button is clicked in inventory screen also sets transfer comp
/// ADVISE - RUN THIS STATELESS  that is why option char collection
fn customizes_character(
    parent: Query<&Parent>,
    mut change_char: EventReader<MessageEvent<ChangeChar>>,
    mut body_part: ResMut<BodyPartMap>,
    client_collection: Option<Res<CharCollection>>,
    gltfs: Res<Assets<Gltf>>,
    mut transfer_anim: EventWriter<TranferAnim>,
    mut commands: Commands,
) {
    for part_to_adjust in change_char.read() {
        let message = part_to_adjust.message();
        let (client_id, part_to_change) = message.0.clone();
        info!("We should adjust client_id {}", client_id);
        info!(
            "Received new parts from inv screen {}",
            part_to_change.old_part
        );
        info!(
            "Received old parts from inv screen {}",
            part_to_change.new_part
        );
        if let Some(old_body_part) = body_part.0.remove(&(client_id, part_to_change.old_part)) {
            info!("Found old body part in map removing it");
            let player = parent
                .get(old_body_part)
                .expect("To always have a father")
                .get();
            commands.entity(old_body_part).despawn_recursive();

            if let Some(ref char_collection) = client_collection {
                let scene = spawn_scene(
                    &part_to_change.new_part,
                    &char_collection,
                    &gltfs,
                    &mut commands,
                );
                info!("Setting father of new part to player");
                commands.entity(scene).set_parent(player);

                info!("Inserting in resource");
                body_part
                    .0
                    .insert((client_id, part_to_change.new_part.clone()), scene);

                info!("Sending transfer anim event to avoid desyncs between parts");
                transfer_anim.send(TranferAnim(client_id));
            } else {
                error!(
                    "Congratulation you manage to access the resource before it is even possible"
                );
            }
        }
    }
}

/// Transfer the animations targets to all the visual bones
fn transfer_essential_components(
    mut body_part_map: ResMut<BodyPartMap>,
    animation_target: Query<&AnimationTarget>,
    children_entities: Query<&Children>,
    names: Query<&Name>,
    has_transfered: Query<&HasTarget>,
    skeleton_map: Res<SkeletonMap>,
    mut read_transfer_anim: EventReader<TranferAnim>,
    mut reset_anim: EventWriter<ResetAnimation>,
    mut commands: Commands,
) {
    for event in read_transfer_anim.read() {
        info!("Lets transfer animations");
        let client_id = event.0;
        if let Some(skeleton) = skeleton_map.0.get(&client_id) {
            info!("Grabbing skeleton corresponding to that client_id");
            let old_entity =
                find_child_with_name_containing(&children_entities, &names, &skeleton, "Armature")
                    .expect("Skeleton to have root armature");

            let mut old_bones = HashMap::new();
            collect_bones(&children_entities, &names, &old_entity, &mut old_bones);

            info!("Grabbing bones in visuals entity and making animation targets for them according to old bones ids");

            // Meh i am lazy not gonna filter out entities
            for ((_, _), body_part) in body_part_map.0.iter_mut() {
                if let Ok(_) = has_transfered.get(*body_part) {
                    // info!(
                    //     "This part is already ready for animation not gonna do it again {}",
                    //     file_path
                    // );
                } else {
                    // info!(
                    //     "Transfering components to apply animation to file path {}",
                    //     file_path
                    // );
                    let new_entity = find_child_with_name_containing(
                        &children_entities,
                        &names,
                        &body_part,
                        "Armature",
                    )
                    .expect("Visual to have root armature");

                    let mut new_bones = HashMap::new();
                    collect_bones(&children_entities, &names, &new_entity, &mut new_bones);

                    commands
                        .entity(new_entity)
                        .insert(AnimationPlayer::default());

                    for (name, old_bone) in old_bones.iter() {
                        let old_animation_target = animation_target
                            .get(*old_bone)
                            .expect("To have target if it doesnt well shit");

                        if let Some(corresponding_bone) = new_bones.get(name) {
                            commands
                                .entity(*corresponding_bone)
                                .insert(AnimationTarget {
                                    id: old_animation_target.id,
                                    player: new_entity,
                                });
                        }
                    }
                    commands.entity(*body_part).insert(HasTarget);
                }
            }
            reset_anim.send(ResetAnimation(client_id));
        } else {
            error!("The base skeleton of this {} doesnt exit", client_id);
        }
    }
}

/// Reset animations after transfering animation targets as to avoid desync
fn reset_animation(
    mut animation_players: Query<&mut AnimationPlayer>,
    children_entities: Query<&Children>,
    names: Query<&Name>,
    body_part_map: Res<BodyPartMap>,
    mut reset_anim: EventReader<ResetAnimation>,
) {
    for event in reset_anim.read() {
        let client_id = event.0;
        info!("Reseting animation for client {}", client_id);
        // Iter through scenes that are child of that player find their child recursively that has anim player reset it
        for entity in body_part_map.find_entities_by_client_id(&client_id) {
            if let Some(entity_with_anim_player) = find_child_with_name_containing(
                &children_entities,
                &names,
                &entity,
                "CharacterArmature",
            ) {
                if let Ok(mut anim_player) = animation_players.get_mut(entity_with_anim_player) {
                    anim_player.rewind_all();
                } else {
                    warn!(
                        "Couldnt rewind all anim player for this client or he doenst  have one {}",
                        client_id
                    )
                }
            }
        }
    }
}
