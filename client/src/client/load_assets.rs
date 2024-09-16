//! RESPONSIBILITIES - LOAD ALL ASSETS WHEN GAME STARTS
//! Once loaded we will continue to state UI

use bevy::{
    prelude::*,
    render::{
        mesh::skinning::SkinnedMesh,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::NoFrustumCulling,
        view::RenderLayers,
    },
    utils::HashMap,
};
use bevy_asset_loader::prelude::*;

pub struct LoadingAssetsPlugin;
use crate::client::MyAppState;

impl Plugin for LoadingAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MyAppState>().add_loading_state(
            LoadingState::new(MyAppState::LoadingAssets)
                .continue_to_state(MyAppState::MainMenu)
                .load_collection::<ClientCharCollection>(),
        );
        app.add_systems(Startup, render_to_texture_character);
        app.add_systems(Update, rotator_system);
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

#[derive(Component)]
pub struct CubeWithRenderTexture;

// This will create a cube that solely has the character assets as a texture on it
pub fn render_to_texture_character(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Size of image
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    // This is the specifications of our image that is gonna become a texture
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

    // Resizeing to fill it with zeroes
    image.resize(size);

    // Adding to assets
    let image_handle = images.add(image);

    // This is the layer for the renderes that got texturized
    let first_pass_layer = RenderLayers::layer(1);

    let scene = SceneBundle {
        scene: asset_server.load("characters/black_cube.glb#Scene0"),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        ..default()
    };

    // The scene that oughta to be texturized
    commands.spawn(scene).insert(first_pass_layer.clone());

    // Render to texture camera, renders a character
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                // render before the "main pass" camera
                order: -1,
                target: image_handle.clone().into(),
                clear_color: Color::WHITE.into(),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        first_pass_layer.clone(),
    ));

    // Converting our rendered  image to an texutre
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle.clone()),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

    // FORMING CUBE to see texture
    let cube_handle = meshes.add(Cuboid::new(0.5, 0.5, 0.5));
    commands.spawn((
        PbrBundle {
            mesh: cube_handle,
            material: material_handle,
            transform: Transform::from_xyz(0.0, 0.0, 1.5),
            ..default()
        },
        CubeWithRenderTexture,
    ));

    // // Simple light to see stuff on both
    commands.spawn((
        PointLightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
            ..default()
        },
        RenderLayers::layer(0).with(1),
    ));

    // // this shall be the main camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },));
}

/// Rotates the inner cube (first pass)
fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<CubeWithRenderTexture>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.5 * time.delta_seconds());
        transform.rotate_z(1.3 * time.delta_seconds());
    }
}
