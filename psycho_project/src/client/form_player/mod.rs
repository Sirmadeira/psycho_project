//! RESPONSIBILITIES - HANDLES ALL MODULAR CHARACTERS CREATIONS AND UPDATES LOBBY RTT

use super::MyAppState;
use crate::client::form_player::helpers::*;
use crate::client::load_assets::ClientCharCollection;
use crate::shared::protocol::player_structs::*;
use bevy::animation::AnimationTarget;
use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::render::{mesh::skinning::SkinnedMesh, view::NoFrustumCulling};
use bevy::transform::commands;
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use bevy_panorbit_camera::{ActiveCameraData, PanOrbitCamera};
use lightyear::client::events::MessageEvent;
mod helpers;

pub struct CreateCharPlugin;

impl Plugin for CreateCharPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MyCharState>();
        app.add_systems(Startup, spawn_rtt_camera);
        app.add_systems(Startup, spawn_light_bundle);
        app.add_systems(
            Update,
            form_rtt_character.run_if(in_state(MyAppState::Lobby)),
        );
        app.add_systems(OnEnter(MyCharState::TransferAnimations), transfer_animation);
        app.add_systems(Update, disable_culling);
    }
}

#[derive(Resource)]
pub struct RttImage(pub Handle<Image>);

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
#[reflect(Debug, PartialEq, Hash, Default)]
pub enum MyCharState {
    #[default]
    // Spawns necessary scenes
    FormPlayer,
    // Transfer animation targets
    TransferAnimations,

    Done,
}

// Marker component tells me who is the skeletons
#[derive(Component)]
struct Skeleton;

// Marker components tells me who is the visual
#[derive(Component)]
struct Visual;

// This will make an rtt with an available pan orbit camera pointing at it. And saves the asset  as an material
fn spawn_rtt_camera(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut images: ResMut<Assets<Image>>,
    mut active_cam: ResMut<ActiveCameraData>,
    mut commands: Commands,
) {
    info!("Creating image to texturize");
    let size = Extent3d {
        width: 4096,
        height: 4096,
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    image.resize(size);

    let image_handle = images.add(image);

    let camera_offset = Vec3::new(0.0, 1.5, 3.5);

    let rtt_camera = Camera3dBundle {
        camera: Camera {
            // render before the "main pass" camera so we
            order: -1,
            target: image_handle.clone().into(),
            clear_color: ClearColorConfig::Custom(Color::srgb(0.212, 0.271, 0.31)),
            ..default()
        },
        transform: Transform::from_translation(camera_offset),
        ..default()
    };
    // Important component to let player control
    let pan_orbit = PanOrbitCamera {
        focus: Vec3::new(0.0, 1.0, 0.0),
        zoom_upper_limit: Some(3.5),
        zoom_lower_limit: Some(1.0),
        ..default()
    };

    info!("Spawning camera that it is gonna give us our wanted render");
    // Render to texture camera, renders a character
    let pan_orbit_id = commands.spawn(rtt_camera).insert(pan_orbit).id();

    info!("Adjusting panorbit camera to solely apply to the camera that renders the character");
    let primary_window = windows
        .get_single()
        .expect("There is only ever one primary window");

    active_cam.set_if_neq(ActiveCameraData {
        entity: Some(pan_orbit_id),
        viewport_size: Some(Vec2::new(size.width as f32, size.height as f32)),
        window_size: Some(Vec2::new(primary_window.width(), primary_window.height())),
        // Setting manual to true ensures PanOrbitCameraPlugin will not overwrite this resource
        manual: true,
    });
    commands.insert_resource(RttImage(image_handle));
}

fn spawn_light_bundle(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::srgb(0.98, 0.95, 0.87),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 1.0, 5.0)),
        ..default()
    });
}

// Occurs everytime you login in basically, gives you your current loadout in the save file
pub fn form_rtt_character(
    // Easy way of grabbing
    mut events: EventReader<MessageEvent<PlayerLoadout>>,
    client_collection: Res<ClientCharCollection>,
    gltfs: Res<Assets<Gltf>>,
    mut char_state: ResMut<NextState<MyCharState>>,
    mut commands: Commands,
) {
    for event in events.read() {
        info!("Grabbing saved loadout from server and applying to rtt");
        let player_visuals = &event.message().0;

        // Spawns each scene in current worlds
        for visual in player_visuals.iter_visuals() {
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
        char_state.set(MyCharState::TransferAnimations);
    }
}

// Transfer the animations to all the visual bones
fn transfer_animation(
    skeleton: Query<Entity, With<Skeleton>>,
    visuals: Query<Entity, With<Visual>>,
    animation_target: Query<&AnimationTarget>,
    children_entities: Query<&Children>,
    names: Query<&Name>,
    mut commands: Commands,
) {
    let skeleton = skeleton
        .get_single()
        .expect("For a client to solely have one single skeleton");

    info!("Grabbing old skeleton bones");
    let old_entity =
        find_child_with_name_containing(&children_entities, &names, &skeleton, "Armature")
            .expect("Armature 1");

    let mut old_bones = HashMap::new();
    collect_bones(&children_entities, &names, &old_entity, &mut old_bones);

    info!("Grabbing bones in visuals entity and making animation targets for them according to old bones ids");
    for visual in visuals.iter() {
        let new_entity =
            find_child_with_name_containing(&children_entities, &names, &visual, "Armature")
                .expect("Armature 2");

        commands
            .entity(new_entity)
            .insert(AnimationPlayer::default());

        let mut new_bones = HashMap::new();
        collect_bones(&children_entities, &names, &new_entity, &mut new_bones);

        for (name, entity) in old_bones.iter() {
            let old_animation_target = animation_target
                .get(*entity)
                .expect("To have target if it doesnt well shit");

            if let Some(new_match_entity) = new_bones.get(name) {
                commands.entity(*new_match_entity).insert(AnimationTarget {
                    id: old_animation_target.id,
                    player: new_entity,
                });
            }
        }
    }
}

// Despawns uncessary old skeleton
pub fn despawn_old_bones(
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

// Sets bones in place of original skeleton
pub fn finish_player(visuals: Query<Entity, With<Visual>>) {
    for visual in visuals.iter() {}
}

// Constructs final skeleton entity - Makes visual armatures child of it and parents  weapons correctly. Also despawn old armatures
// pub fn make_end_entity(
//     skeleton: Query<(Entity, &Attachments), With<Skeleton>>,
//     children_entities: Query<&Children>,
//     names: Query<&Name>,
//     mut transforms: Query<&mut Transform>,
//     mut commands: Commands,
//     mut next_state: ResMut<NextState<MyCharState>>,
// ) {
//     for (skeleton, attachments) in skeleton.iter() {
//         // This isnt despawned earlier because of apply_deffered
//         let old_base_armature =
//             find_child_with_name_containing(&children_entities, &names, &skeleton, "Armature")
//                 .expect("Old armature");

//         commands.entity(old_base_armature).despawn_recursive();

//         for attachment in attachments.visual.iter() {
//             if let Some(visual_attachment) = attachment {
//                 commands
//                     .entity(*visual_attachment)
//                     .set_parent_in_place(skeleton);

//                 for attachment in attachments.weapons.iter() {
//                     if let Some(weapon_attachment) = attachment {
//                         if let Some(handle_gun) = find_child_with_name_containing(
//                             &children_entities,
//                             &names,
//                             &visual_attachment,
//                             "Handle",
//                         ) {
//                             // Adjusting transform
//                             commands.entity(*weapon_attachment).set_parent(handle_gun);

//                             let mut transform = transforms
//                                 .get_mut(*weapon_attachment)
//                                 .expect("Transform to apply offset");

//                             let amount: f32 = -180.0;
//                             transform.rotation =
//                                 Quat::from_axis_angle(Vec3::X, amount.to_radians());
//                         } else {
//                             println!("The visual bone {} didn't have a handle", visual_attachment);
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     next_state.set(MyCharState::Done);
// }

// Debugger function in animations
pub fn disable_culling(mut commands: Commands, skinned: Query<Entity, Added<SkinnedMesh>>) {
    for entity in &skinned {
        commands.entity(entity).insert(NoFrustumCulling);
    }
}
