//! RESPONSIBILITIES - LOAD ALL ASSETS WHEN GAME STARTS
//! Once loaded we will continue to state UI

use bevy::math::VectorSpace;
use bevy::render::{mesh::skinning::SkinnedMesh, view::NoFrustumCulling};
use bevy::window::PrimaryWindow;
use bevy::{
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    utils::HashMap,
};
use bevy_asset_loader::prelude::*;
use bevy_panorbit_camera::{ActiveCameraData, PanOrbitCamera};

pub struct LoadingAssetsPlugin;
use crate::client::MyAppState;

impl Plugin for LoadingAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MyAppState>().add_loading_state(
            LoadingState::new(MyAppState::LoadingAssets)
                .continue_to_state(MyAppState::MainMenu)
                .load_collection::<ClientCharCollection>(),
        );
        app.add_systems(Update, disable_culling);
    }
}

// Resource for easily acessing client based assets, which are mostly things like character world and so on. Each field in the connect is gonna be associate with something.
#[derive(AssetCollection, Resource)]
pub struct ClientCharCollection {
    #[asset(
        paths("characters/character_mesh.glb", "weapons/katana.glb"),
        collection(typed, mapped)
    )]
    pub gltf_files: HashMap<String, Handle<Gltf>>,
}

// This will create a cube that solely has the character assets as a texture on it
pub fn render_to_texture_character(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    gltfs: ResMut<Assets<Gltf>>,
    mut active_cam: ResMut<ActiveCameraData>,
    client_collection: Res<ClientCharCollection>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    info!("Creating image to texturize");
    let size = Extent3d {
        width: 512,
        height: 512,
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

    info!("Grabbing gltf file for character and spawning him");
    let gltf = client_collection
        .gltf_files
        .get("characters/character_mesh.glb")
        .expect("Gltf to be loaded");

    let loaded_gltf = gltfs
        .get(gltf)
        .expect("To find gltf handle in loaded gltfs");

    let character_scene = loaded_gltf.scenes[0].clone();

    let camera_offset = Vec3::new(0.0, 3.0, 2.5);

    let char_position = Vec3::new(8.0, 0.0, 0.0);

    let rtt_camera = Camera3dBundle {
        camera: Camera {
            // render before the "main pass" camera so we
            order: -1,
            target: image_handle.clone().into(),
            clear_color: Color::WHITE.into(),
            ..default()
        },
        transform: Transform::from_translation(camera_offset),
        ..default()
    };

    let scene = SceneBundle {
        scene: character_scene,
        transform: Transform::from_translation(char_position)
            // If you want him to stare face front to camera
            .looking_at(Vec3::new(0.0, 0.0, 1.0), Vec3::Y),
        ..default()
    };

    // The scene that oughta to be texturized
    commands.spawn(scene);

    info!("Spawning camera that it is gonna give us our wanted render");
    // Render to texture camera, renders a character
    let pan_orbit_id = commands
        .spawn(rtt_camera)
        .insert(PanOrbitCamera::default())
        .id();

    // Converting our rendered  image to an texture
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle.clone()),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

    // Simple light to see stuff on both
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 1.0, 5.0)),
        ..default()
    });

    // this shall be the main camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },));

    info!("Adjusting panorbit camera to solely apply to the camera that renders the character");
    let primary_window = windows
        .get_single()
        .expect("There is only ever one primary window");

    active_cam.set_if_neq(ActiveCameraData {
        entity: Some(pan_orbit_id),
        // What you set these values to will depend on your use case, but generally you want the
        // viewport size to match the size of the render target (image, viewport), and the window
        // size to match the size of the window that you are interacting with.
        viewport_size: Some(Vec2::new(size.width as f32, size.height as f32)),
        window_size: Some(Vec2::new(primary_window.width(), primary_window.height())),
        // Setting manual to true ensures PanOrbitCameraPlugin will not overwrite this resource
        manual: true,
    });
}

// Debugger function in animations
pub fn disable_culling(mut commands: Commands, skinned: Query<Entity, Added<SkinnedMesh>>) {
    for entity in &skinned {
        commands.entity(entity).insert(NoFrustumCulling);
    }
}
