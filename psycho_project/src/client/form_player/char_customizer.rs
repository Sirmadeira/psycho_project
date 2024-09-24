//! Plugin responsible for customizing the player character in rtt and the final result shall be used and replicated when enter ingame state
use crate::client::load_assets::ClientCharCollection;
use crate::client::{form_player::helpers::*, MyAppState};
use crate::shared::protocol::player_structs::*;
use bevy::animation::AnimationTarget;
use bevy::prelude::*;
use bevy::utils::HashMap;
use lightyear::shared::replication::components::{Controlled, Replicated};

pub struct CustomizeChar;

impl Plugin for CustomizeChar {
    fn build(&self, app: &mut App) {
        app.init_state::<MyCharState>();
        app.add_systems(Update, form_character.run_if(is_loaded));
        app.add_systems(
            OnEnter(MyCharState::TransferComp),
            transfer_essential_components,
        );
        app.add_systems(OnExit(MyCharState::TransferComp), despawn_old_bones);
    }
}

// Rc - Only run this system if it has all assets available
fn is_loaded(state: Res<State<MyAppState>>) -> bool {
    if *state != MyAppState::LoadingAssets {
        return true;
    } else {
        return false;
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
#[reflect(Debug, PartialEq, Hash, Default)]
pub enum MyCharState {
    #[default]
    // Spawns necessary scenes
    FormPlayer,
    // Transfer animation targets
    TransferComp,
    // Making final replicated entity
    Replicating,
    Done,
}

// Marker component tells me who is the skeletons
#[derive(Component)]
struct Skeleton;

// Marker components tells me who is the visual
#[derive(Component)]
struct Visual;

// Occurs everytime a player is replicated in basically, gives you your current loadout in the save file
pub fn form_character(
    player_to_form: Query<&PlayerId, (Added<Replicated>, Added<Controlled>)>,
    client_collection: Res<ClientCharCollection>,
    gltfs: Res<Assets<Gltf>>,
    mut char_state: ResMut<NextState<MyCharState>>,
    bundle_map: Res<PlayerBundleMap>,
    mut commands: Commands,
) {
    if let Ok(player) = player_to_form.get_single() {
        info!("Grabbing saved loadout from server and applying to rtt");
        let client_id = player.0;
        if let Some(player_info) = bundle_map.0.get(&client_id) {
            info!("Grabbing stored visuals in replicated resource");
            let visuals = &player_info.visuals;
            for visual in visuals.iter_visuals() {
                let gltf = client_collection
                    .gltf_files
                    .get(visual)
                    .expect("Gltf to be loaded");

                let loaded_gltf = gltfs
                    .get(gltf)
                    .expect("To find gltf handle in loaded gltfs");

                let visual_scene = loaded_gltf.scenes[0].clone();
                let char_position = Vec3::new(0.0, 0.0, 0.0);
                let scene = SceneBundle {
                    scene: visual_scene,
                    transform: Transform::from_translation(char_position),
                    // If you want him to stare face front to camera as from blender he usually stares at negative -z
                    // .looking_at(Vec3::new(0.0, 0.0, 1.0), Vec3::Y),
                    ..default()
                };
                info!("Spawning scene entities to be utilized in creating our character");
                if visual.contains("skeleton") {
                    info!("Spawning and marking main skeleton entity");
                    commands.spawn((Skeleton, scene));
                } else {
                    info!("Spawning visual {} scene", visual);
                    commands.spawn((Visual, scene));
                }
            }
            char_state.set(MyCharState::TransferComp);
        }
    }
}

// Transfer the animations to all the visual bones
fn transfer_essential_components(
    skeleton: Query<Entity, With<Skeleton>>,
    visuals: Query<Entity, With<Visual>>,
    animation_target: Query<&AnimationTarget>,
    children_entities: Query<&Children>,
    names: Query<&Name>,
    mut char_state: ResMut<NextState<MyCharState>>,
    mut commands: Commands,
) {
    let skeleton = skeleton
        .get_single()
        .expect("For a client to solely have one single skeleton");

    info!("Grabbing old skeleton bones");
    let old_entity =
        find_child_with_name_containing(&children_entities, &names, &skeleton, "Armature")
            .expect("Skeleton to have root armature");

    let mut old_bones = HashMap::new();
    collect_bones(&children_entities, &names, &old_entity, &mut old_bones);

    info!("Grabbing bones in visuals entity and making animation targets for them according to old bones ids");
    for visual in visuals.iter() {
        let new_entity =
            find_child_with_name_containing(&children_entities, &names, &visual, "Armature")
                .expect("Visual to have root armature");

        commands
            .entity(new_entity)
            .insert(AnimationPlayer::default());

        let mut new_bones = HashMap::new();
        collect_bones(&children_entities, &names, &new_entity, &mut new_bones);

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
    }
    char_state.set(MyCharState::Replicating);
}

// Despawns uncessary old skeleton- Lots of entities here so extremely uneeded
fn despawn_old_bones(
    skeleton: Query<Entity, With<Skeleton>>,
    mut commands: Commands,
    children_entities: Query<&Children>,
    names: Query<&Name>,
) {
    let skeleton = skeleton
        .get_single()
        .expect("For player to solely have one skeleton");

    info!("Despawning unecessary old armature");
    let old_base_armature =
        find_child_with_name_containing(&children_entities, &names, &skeleton, "Armature")
            .expect("Old armature");

    commands.entity(old_base_armature).despawn_recursive();
}
