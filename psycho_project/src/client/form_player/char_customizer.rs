//! Plugin responsible for customizing the player character in rtt and the final result shall be used and replicated when enter ingame state
use crate::client::form_player::helpers::*;
use crate::client::is_loaded;
use crate::client::load_assets::CharCollection;
use crate::shared::protocol::player_structs::*;
use bevy::animation::AnimationTarget;
use bevy::prelude::*;
use bevy::utils::HashMap;
use lightyear::client::events::MessageEvent;

pub struct CustomizeChar;

impl Plugin for CustomizeChar {
    fn build(&self, app: &mut App) {
        app.init_state::<MyCharState>();
        app.add_systems(Update, form_main_player_character.run_if(is_loaded));
        app.add_systems(
            OnEnter(MyCharState::TransferComp),
            transfer_essential_components,
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

// Marker components tells me who is the visual
#[derive(Component)]
struct Visual;

fn spawn_char(
    player_info: &PlayerBundle,
    client_collection: &Res<CharCollection>,
    gltfs: &Res<Assets<Gltf>>,
    commands: &mut Commands,
) {
    for visual in player_info.visuals.iter_visuals() {
        if let Some(gltf) = client_collection.gltf_files.get(visual) {
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
        } else {
            error!("This gltf doesnt exist or isnt loaded {}", visual);
        }
    }
}

// Forms main player, important to occur before the start game, as customizer and rtt requires it
pub fn form_main_player_character(
    client_collection: Res<CharCollection>,
    gltfs: Res<Assets<Gltf>>,
    mut char_state: ResMut<NextState<MyCharState>>,
    mut events: EventReader<MessageEvent<SendBundle>>,
    mut commands: Commands,
) {
    for event in events.read() {
        let server_bundle = &event.message().0;

        let client_id = server_bundle.id.0;
        info!("Spawning visuals for client_id {}", client_id);

        spawn_char(server_bundle, &client_collection, &gltfs, &mut commands);
        info!("Transfering animations");
        char_state.set(MyCharState::TransferComp);
    }
}

// Transfer the animations to all the visual bones
fn transfer_essential_components(
    skeletons: Query<Entity, With<Skeleton>>,
    visuals: Query<Entity, With<Visual>>,
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
    }
    char_state.set(MyCharState::Done);
}
