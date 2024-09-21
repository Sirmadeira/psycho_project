//! RESPONSIBILITIES - HANDLES ALL MODULAR CHARACTERS CREATIONS

use super::MyAppState;
use crate::client::load_assets::ClientCharCollection;
use crate::shared::protocol::player_structs::*;
use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::render::{mesh::skinning::SkinnedMesh, view::NoFrustumCulling};
use bevy::window::PrimaryWindow;
use bevy_panorbit_camera::{ActiveCameraData, PanOrbitCamera};
use lightyear::client::events::MessageEvent;

pub struct CreateCharPlugin;

impl Plugin for CreateCharPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_rtt_camera);
        app.add_systems(
            Update,
            form_rtt_character.run_if(in_state(MyAppState::Lobby)),
        );
        app.add_systems(Update, disable_culling);
    }
}

#[derive(Resource)]
pub struct RttImage(pub Handle<Image>);

// This will make an rtt with an available pan orbit camera pointing at it. And saves the asset  as an material
pub fn spawn_rtt_camera(
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

pub fn form_rtt_character(
    // Easy way of grabbing
    mut events: EventReader<MessageEvent<PlayerLoadout>>,
    client_collection: Res<ClientCharCollection>,
    gltfs: Res<Assets<Gltf>>,
    mut commands: Commands,
) {
    for event in events.read() {
        info!("Grabbing saved loadout from server and applying to rtt");
        let player_visual = &event.message().0.character;

        let gltf = client_collection
            .gltf_files
            .get(player_visual)
            .expect("Gltf to be loaded");

        let loaded_gltf = gltfs
            .get(gltf)
            .expect("To find gltf handle in loaded gltfs");

        let character_scene = loaded_gltf.scenes[0].clone();

        let char_position = Vec3::new(0.0, 0.0, 0.0);

        let scene = SceneBundle {
            scene: character_scene,
            transform: Transform::from_translation(char_position)
                // If you want him to stare face front to camera as from blender he usually stares at negative -z
                .looking_at(Vec3::new(0.0, 0.0, 1.0), Vec3::Y),
            ..default()
        };

        info!("Spawning character rtt scene");
        commands.spawn(scene);

        // Simple light to see stuff on both
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
}

// Debugger function in animations
pub fn disable_culling(mut commands: Commands, skinned: Query<Entity, Added<SkinnedMesh>>) {
    for entity in &skinned {
        commands.entity(entity).insert(NoFrustumCulling);
    }
}

// TODO - UI FOR THIS for now client only sends the default value
// pub(crate) fn insert_visuals(player_id: Query<Entity, With<PlayerId>>) {}

// This will spawn our main characters according TO THE AMOUNT OF ENTITIES, IN LOBBY. TODO LOBBY
// pub(crate) fn spawn_character(
//     player: Query<Entity, With<Predicted>>,
//     client_collection: Res<ClientCharCollection>,
//     assets_gltf: Res<Assets<Gltf>>,
//     mut commands: Commands,
// ) {
//     for _ in player.iter() {
//         info!("All players being created");
//         for (file_name, han_gltf) in &client_collection.gltf_files {
//             if file_name.contains("character_mesh") {
//                 // Loading gltf from asset_server
//                 let gltf_scene = assets_gltf
//                     .get(han_gltf)
//                     .expect("The handle in server to be loaded");

//                 // Grabbng mesh
//                 let player_mesh = SceneBundle {
//                     scene: gltf_scene.named_scenes["Scene"].clone(),
//                     transform: Transform::from_xyz(0.0, 0.0, 0.0),
//                     ..Default::default()
//                 };

//                 commands.spawn(player_mesh);
//             }
//         }
//     }
// }
