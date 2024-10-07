//! Plugin responsible for customizing the player character in rtt and the final result shall be used and replicated when enter ingame state
use crate::client::form_player::helpers::*;
use crate::client::load_assets::CharCollection;
use crate::client::ui::inventory_screen::ChangeChar;
use crate::client::MyAppState;
use crate::shared::protocol::player_structs::*;
use bevy::animation::AnimationTarget;
use bevy::math::bounding::BoundingVolume;
use bevy::prelude::*;
use bevy::utils::HashMap;
use lightyear::client::interpolation::Interpolated;

use crate::client::essentials::EasyClient;

pub struct CustomizeCharPlugin;

impl Plugin for CustomizeCharPlugin {
    fn build(&self, app: &mut App) {
        // States
        app.init_state::<MyCharState>();
        //Debugging
        app.register_type::<BodyPartMap>();

        app.add_systems(OnEnter(MyAppState::MainMenu), form_main_player_character);
        app.add_systems(
            OnEnter(MyCharState::TransferComp),
            transfer_essential_components,
        );

        app.add_systems(
            Update,
            customizes_character.run_if(in_state(MyCharState::Done)),
        );

        // Observer systems
        app.observe(form_side_player);

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

/// Marker component tells me who is the skeletons
#[derive(Component)]
pub struct Skeleton;

/// Resource that tell me which assets had their animation targets transfered
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct BodyPartMap(pub HashMap<String, Entity>);

/// A simple component that tells me if it already transfered the animation targets
#[derive(Component)]
struct HasTarget;

// Helper function spawns a series of scenes acording to the given batch of visuals being passed
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

    info!("Spawning visual {} scene", visual);

    // Spawn and return the appropriate entity
    if visual.contains("skeleton") {
        info!("Spawning and marking main skeleton entity");
        let id = commands.spawn((Skeleton, scene)).id();
        id
    } else {
        let id = commands.spawn(scene).id();
        id
    }
}

/// Forms main player, according to the bundle replicated from server, important to have for RTTs.And because server should be the one controlling it no replication for you
pub fn form_main_player_character(
    client_collection: Res<CharCollection>,
    bundle_map: Res<PlayerBundleMap>,
    easy_client: Option<Res<EasyClient>>,
    gltfs: Res<Assets<Gltf>>,
    mut char_state: ResMut<NextState<MyCharState>>,
    mut commands: Commands,
) {
    if let Some(easy_client) = easy_client {
        if let Some(server_bundle) = bundle_map.0.get(&easy_client.0) {
            let client_id = server_bundle.id.0;
            info!("Spawning visuals for client_id {}", client_id);

            let mut hash_map = HashMap::new();
            for visual in server_bundle.visuals.iter_visuals() {
                let id = spawn_scene(visual, &client_collection, &gltfs, &mut commands);
                hash_map.insert(visual.to_string(), id);
            }
            commands.insert_resource(BodyPartMap(hash_map));

            info!("Transfering animations");
            char_state.set(MyCharState::TransferComp);
        }
    } else {
        warn!("You are not connected no character for you !")
    }
}

/// Only occurs when there is interpolated entities and replication occurs
fn form_side_player(
    trigger: Trigger<OnInsert, PlayerVisuals>,
    scenes_to_load: Query<&PlayerVisuals, With<Interpolated>>,
    gltfs: Res<Assets<Gltf>>,
    client_collection: Res<CharCollection>,
    mut body_part_map: ResMut<BodyPartMap>,
    mut char_state: ResMut<NextState<MyCharState>>,
    mut commands: Commands,
) {
    let side_player = trigger.entity();
    if let Ok(player_visuals) = scenes_to_load.get(side_player) {
        info_once!("Spawning side player character and mapping him for transfering animations and marking his body parts");
        for visual in player_visuals.iter_visuals() {
            let id = spawn_scene(visual, &client_collection, &gltfs, &mut commands);
            body_part_map
                .0
                .insert(format!("side_{}", visual.to_string()), id);
        }
    } else {
        error!("Couldnt grab player visuals {:?}", side_player);
    }
    info!("Transfering animations for side player");
    char_state.set(MyCharState::TransferComp);
}

/// Customizes character after a button is clicked in inventory screen also sets transfer comp
fn customizes_character(
    mut change_char: EventReader<ChangeChar>,
    mut body_part: ResMut<BodyPartMap>,
    client_collection: Res<CharCollection>,
    gltfs: Res<Assets<Gltf>>,
    mut char_state: ResMut<NextState<MyCharState>>,
    mut commands: Commands,
) {
    for part_to_adjust in change_char.read() {
        info!(
            "Received parts from inv screen {}",
            part_to_adjust.0.old_part
        );
        info!(
            "Received parts from inv screen {}",
            part_to_adjust.0.new_part
        );
        if let Some(old_part) = body_part.0.remove(&part_to_adjust.0.old_part) {
            info!("This dude needs to die {:?}", old_part);
            commands.entity(old_part).despawn_recursive();
        }

        info!(
            "This dude need to exist {}",
            part_to_adjust.0.new_part.clone()
        );
        let scene_id = spawn_scene(
            &part_to_adjust.0.new_part,
            &client_collection,
            &gltfs,
            &mut commands,
        );

        body_part
            .0
            .insert(part_to_adjust.0.new_part.clone(), scene_id);

        char_state.set(MyCharState::TransferComp);
    }
}

/// Transfer the animations to all the visual bones
fn transfer_essential_components(
    skeletons: Query<Entity, With<Skeleton>>,
    mut visuals: ResMut<BodyPartMap>,
    animation_target: Query<&AnimationTarget>,
    children_entities: Query<&Children>,
    names: Query<&Name>,
    has_transfered: Query<&HasTarget>,
    mut char_state: ResMut<NextState<MyCharState>>,
    mut commands: Commands,
) {
    for skeleton in skeletons.iter() {
        info!("Grabbing old skeleton bones");
        let old_entity =
            find_child_with_name_containing(&children_entities, &names, &skeleton, "Armature")
                .expect("Skeleton to have root armature");

        let mut old_bones = HashMap::new();
        collect_bones(&children_entities, &names, &old_entity, &mut old_bones);

        info!("Grabbing bones in visuals entity and making animation targets for them according to old bones ids");
        for (file_path, visual) in visuals.0.iter_mut() {
            if let Ok(_) = has_transfered.get(*visual) {
                info!(
                    "This part is already ready for animation not gonna do it again {}",
                    file_path
                );
            } else {
                info!(
                    "Transfering components to apply animation to file path {}",
                    file_path
                );
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
    }
    char_state.set(MyCharState::Done);
}

/// Reset animations after transfering animation targets as to avoid desync
fn reset_animation(mut animation_players: Query<&mut AnimationPlayer, With<AnimationPlayer>>) {
    for mut animation_player in animation_players.iter_mut() {
        animation_player.rewind_all();
    }
}
