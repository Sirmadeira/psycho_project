//! Plugin responsible for customizing the player character in rtt and the final result shall be used and replicated when enter ingame state
use crate::client::form_player::helpers::*;
use crate::client::load_assets::CharCollection;
use crate::client::ui::inventory_screen::ChangeChar;
use crate::client::MyAppState;
use crate::shared::protocol::player_structs::*;
use bevy::animation::AnimationTarget;
use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::client::essentials::EasyClient;

pub struct CustomizeChar;

impl Plugin for CustomizeChar {
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
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
#[reflect(Debug, PartialEq, Hash, Default)]
pub enum MyCharState {
    #[default]
    // Spawns necessary scenes
    FormingPlayers,
    // Transfer animation targets
    TransferComp,
    Done,
}

// Marker component tells me who is the skeletons
#[derive(Component)]
pub struct Skeleton;

// Marker components tells me who is the visual and if it is animation was transferred
#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct BodyPartMap(HashMap<String, (Entity, bool)>);

fn spawn_char(
    visual: &str,
    client_collection: &Res<CharCollection>,
    gltfs: &Res<Assets<Gltf>>,
    commands: &mut Commands,
) -> (Entity, bool) {
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
        (id, false)
    } else {
        let id = commands.spawn(scene).id();
        (id, false)
    }
}

// Forms main player, according to the bundle replicated from server, important to have for RTTs
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
                let (id, is_mapped) = spawn_char(visual, &client_collection, &gltfs, &mut commands);
                hash_map.insert(visual.to_string(), (id, is_mapped));
            }
            commands.insert_resource(BodyPartMap(hash_map));

            info!("Transfering animations");
            char_state.set(MyCharState::TransferComp);
        }
    } else {
        warn!("You are not connected no character for you !")
    }
}

// Customizes character after a button is clicked in inventory screen also sets transfer comp
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
        if let Some((old_part, _)) = body_part.0.remove(&part_to_adjust.0.old_part) {
            info!("This dude needs to die {:?}", old_part);
            commands.entity(old_part).despawn_recursive();
        }

        info!(
            "This dude need to exist {}",
            part_to_adjust.0.new_part.clone()
        );
        let (scene_id, is_mapped) = spawn_char(
            &part_to_adjust.0.new_part,
            &client_collection,
            &gltfs,
            &mut commands,
        );

        body_part
            .0
            .insert(part_to_adjust.0.new_part.clone(), (scene_id, is_mapped));

        char_state.set(MyCharState::TransferComp);
    }
}

// Transfer the animations to all the visual bones
fn transfer_essential_components(
    skeletons: Query<Entity, With<Skeleton>>,
    mut visuals: ResMut<BodyPartMap>,
    animation_target: Query<&AnimationTarget>,
    children_entities: Query<&Children>,
    names: Query<&Name>,
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
        for (file_path, (visual, is_mapped)) in visuals.0.iter_mut() {
            if !*is_mapped {
                info!(
                    "Transfering components to apply animation to file path {}",
                    file_path
                );
                *is_mapped = true;
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
            } else {
                info!(
                    "This part was already transfered not gonna do it again {}",
                    file_path
                );
            }
        }
    }
    char_state.set(MyCharState::Done);
}
