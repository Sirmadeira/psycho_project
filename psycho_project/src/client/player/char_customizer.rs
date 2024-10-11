//! Plugin responsible for customizing the player character in rtt and the final result shall be used and replicated when enter ingame state
use crate::client::essentials::EasyClient;
use crate::client::load_assets::CharCollection;
use crate::client::player;
use crate::client::ui::inventory_screen::ChangeChar;
use crate::shared::protocol::player_structs::*;
use bevy::animation::AnimationTarget;
use bevy::prelude::*;
use bevy::utils::HashMap;
use lightyear::client::interpolation::Interpolated;
use lightyear::connection::id::ClientId;
use lightyear::prelude::client::Predicted;
use std::collections::VecDeque;

pub struct CustomizeCharPlugin;

impl Plugin for CustomizeCharPlugin {
    fn build(&self, app: &mut App) {
        //Events
        app.add_event::<TranferAnim>();

        // States
        app.init_state::<MyCharState>();

        // Starting up base resource
        app.init_resource::<BodyPartMap>();
        app.init_resource::<SkeletonMap>();

        //Debugging
        app.register_type::<BodyPartMap>();
        app.register_type::<SkeletonMap>();
        app.register_type::<MyCharState>();

        // Observes when to create players
        app.observe(spawn_main_player);
        app.observe(spawn_side_player);

        app.add_systems(
            OnEnter(MyCharState::TransferComp),
            transfer_essential_components,
        );

        // app.add_systems(
        //     Update,
        //     customizes_character.run_if(in_state(MyCharState::Done)),
        // );

        // Observer systems
        app.add_systems(OnEnter(MyCharState::Done), reset_animation);
    }
}

/// State that resets everytime a new scene that need to be animated is inserted
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
#[reflect(Debug, PartialEq, Hash, Default)]
pub enum MyCharState {
    #[default]
    // Spawns necessary scenes
    FormingPlayers,
    // Transfer animation targets
    TransferComp,
    // Reset animations
    Done,
}


/// Resource that tell me which assets had their animation targets transfered
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct BodyPartMap(pub HashMap<(ClientId, String), Entity>);

/// Tell me who is the current skeleton of that player
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
struct SkeletonMap(HashMap<ClientId, Entity>);

/// Tell me when to transfer the anim of a certain player
#[derive(Event, Reflect)]
struct TranferAnim(ClientId);

/// A simple component that tells me if it already transfered the animation targets
#[derive(Component)]
struct HasTarget;

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
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
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
pub fn find_child_with_name_containing(
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
fn spawn_main_player(
    trigger: Trigger<OnInsert, Predicted>,
    easy_client: Res<EasyClient>,
    player_bundle_map: Res<PlayerBundleMap>,
    gltfs: Res<Assets<Gltf>>,
    client_collection: Res<CharCollection>,
    mut body_part_map: ResMut<BodyPartMap>,
    mut skeleton_map: ResMut<SkeletonMap>,
    mut transfer_anim: EventWriter<TranferAnim>,
    mut commands: Commands,
) {
    let main_player = trigger.entity();
    let client_id = easy_client.0;
    if let Some(player_bundle) = player_bundle_map.0.get(&client_id) {
        let main_player_scenes = player_bundle.visuals.clone();

        info!("Formulating main player entitty");
        let main_player = commands
            .entity(main_player)
            .insert(SpatialBundle {
                transform: Transform::from_xyz(0.5, 0.0, 0.0),
                ..default()
            })
            .insert(Name::new("MainPlayer"))
            .id();

        info!("Spawning it is visuals and setting him as parent and inserting into body part map");
        for file_path in main_player_scenes.iter_visuals() {
            if file_path.contains("skeleton") {
                let visual_scene =
                    spawn_scene(&file_path, &client_collection, &gltfs, &mut commands);
                commands.entity(visual_scene).set_parent(main_player);
                info!("Inserting skeleton into map");
                skeleton_map.0.insert(client_id, visual_scene);
            } else {
                let visual_scene =
                    spawn_scene(&file_path, &client_collection, &gltfs, &mut commands);
                commands.entity(visual_scene).set_parent(main_player);

                body_part_map
                    .0
                    .insert((client_id, file_path.to_string()), visual_scene);
            }
        }
        info!("Telling him to transfer animation targets according to his skeleton");
        transfer_anim.send(TranferAnim(client_id));
    }
}

/// Spawns visual scenes and parents them to interpolated players
fn spawn_side_player(
    trigger: Trigger<OnInsert, PlayerVisuals>,
    scenes_to_load: Query<(&PlayerId, &PlayerVisuals), With<Interpolated>>,
    gltfs: Res<Assets<Gltf>>,
    client_collection: Res<CharCollection>,
    mut body_part_map: ResMut<BodyPartMap>,
    mut skeleton_map: ResMut<SkeletonMap>,
    mut transfer_anim: EventWriter<TranferAnim>,
    mut commands: Commands,
) {
    let side_player = trigger.entity();
    // Check if it is the interpolated player - WORTH noting interpolated is inserted first them player visuals
    if let Ok((player_id, player_visuals)) = scenes_to_load.get(side_player) {
        let client_id = player_id.0;
        info!("Inserting additonal info  component in interpolated player");
        commands
            .entity(side_player)
            .insert(SpatialBundle::default())
            .insert(Name::new("SidePlayer"));
        for file_path in player_visuals.iter_visuals() {
            if file_path.contains("skeleton") {
                info!("Found side player skeleton");
                let visual_scene =
                    spawn_scene(&file_path, &client_collection, &gltfs, &mut commands);
                commands.entity(visual_scene).set_parent(side_player);

                info!("Inserting skeleton into map");
                skeleton_map.0.insert(client_id, visual_scene);
            } else {
                let visual_scene =
                    spawn_scene(&file_path, &client_collection, &gltfs, &mut commands);

                commands.entity(visual_scene).set_parent(side_player);

                body_part_map
                    .0
                    .insert((client_id, file_path.to_string()), visual_scene);
            }
        }
        info!("Telling him to transfer animation targets according to his skeleton");
        transfer_anim.send(TranferAnim(client_id));
    }
}

/// Customizes character after a button is clicked in inventory screen also sets transfer comp
// fn customizes_character(
//     parent: Query<&Parent>,
//     mut change_char: EventReader<ChangeChar>,
//     mut body_part: ResMut<BodyPartMap>,
//     client_collection: Res<CharCollection>,
//     gltfs: Res<Assets<Gltf>>,
//     mut char_state: ResMut<NextState<MyCharState>>,
//     mut commands: Commands,
// ) {
//     for part_to_adjust in change_char.read() {
//         info!(
//             "Received parts from inv screen {}",
//             part_to_adjust.0.old_part
//         );
//         info!(
//             "Received parts from inv screen {}",
//             part_to_adjust.0.new_part
//         );
//         if let Some(old_part) = body_part.0.remove(&part_to_adjust.0.old_part) {
//             info!("This dude needs to die {:?}", old_part);
//             commands.entity(old_part).despawn_recursive();
//             if let Ok(parent) = parent.get(old_part) {
//                 let player = parent.get();
//                 info!(
//                     "This dude need to exist {}",
//                     part_to_adjust.0.new_part.clone()
//                 );
//                 let scene_id = spawn_scene(
//                     &part_to_adjust.0.new_part,
//                     &client_collection,
//                     &gltfs,
//                     &mut commands,
//                 );

//                 info!("Inserting new body part into general map");
//                 body_part
//                     .0
//                     .insert(part_to_adjust.0.new_part.clone(), scene_id);
//                 info!("Set player as parent of visual");
//                 commands.entity(scene_id).set_parent(player);

//                 info!("Changin state to transfer anim target");
//                 char_state.set(MyCharState::TransferComp);
//             }
//         }
//     }
// }

/// Transfer the animations to all the visual bones
fn transfer_essential_components(
    mut visuals: ResMut<BodyPartMap>,
    animation_target: Query<&AnimationTarget>,
    children_entities: Query<&Children>,
    names: Query<&Name>,
    has_transfered: Query<&HasTarget>,
    mut char_state: ResMut<NextState<MyCharState>>,
    skeleton_map: Res<SkeletonMap>,
    mut read_transfer_anim: EventReader<TranferAnim>,
    mut commands: Commands,
) {
    for event in read_transfer_anim.read() {
        let client_id = event.0;
        if let Some(skeleton) = skeleton_map.0.get(&client_id) {
            info!("Grabbing old skeleton bones");
            let old_entity =
                find_child_with_name_containing(&children_entities, &names, &skeleton, "Armature")
                    .expect("Skeleton to have root armature");

            let mut old_bones = HashMap::new();
            collect_bones(&children_entities, &names, &old_entity, &mut old_bones);

            info!("Grabbing bones in visuals entity and making animation targets for them according to old bones ids");
            for (_, visual) in visuals.0.iter_mut() {
                if let Ok(_) = has_transfered.get(*visual) {
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
                        &visual,
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
                    commands.entity(*visual).insert(HasTarget);
                }
            }
        } else {
            error!("The base skeleton of this {} doesnt exit", client_id);
        }
    }
    char_state.set(MyCharState::Done);
}

/// Reset animations after transfering animation targets as to avoid desync
fn reset_animation(mut animation_players: Query<&mut AnimationPlayer, With<AnimationPlayer>>) {
    for mut animation_player in animation_players.iter_mut() {
        animation_player.rewind_all();
    }
}
