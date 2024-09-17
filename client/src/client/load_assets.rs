//! RESPONSIBILITIES - LOAD ALL ASSETS WHEN GAME STARTS
//! Once loaded we will continue to state UI

use bevy::math::VectorSpace;
use bevy::render::{mesh::skinning::SkinnedMesh, view::NoFrustumCulling};
use bevy::{
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    utils::HashMap,
};
use bevy_asset_loader::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

pub struct LoadingAssetsPlugin;
use crate::client::MyAppState;

impl Plugin for LoadingAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MyAppState>().add_loading_state(
            LoadingState::new(MyAppState::LoadingAssets)
                .continue_to_state(MyAppState::MainMenu)
                .load_collection::<ClientCharCollection>(),
        );
        app.add_systems(OnEnter(MyAppState::MainMenu), render_to_texture_character);
        app.add_systems(Update, rotator_system);
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

#[derive(Component)]
pub struct CubeWithRenderTexture;

// This will create a cube that solely has the character assets as a texture on it
pub fn render_to_texture_character(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    gltfs: ResMut<Assets<Gltf>>,
    client_collection: Res<ClientCharCollection>,
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

    let character_transform = Transform::from_xyz(3.0, 0.0, 3.0);
    // let base_transform = Transform::

    // ATTENTION - THIS IS 100 BECAUSE SCENES ARE VERY ANNOYING TO BE USED WITH RENDER LAYERS
    let scene = SceneBundle {
        scene: character_scene,
        transform: character_transform,
        ..default()
    };

    let rtt_camera = Camera3dBundle {
        camera: Camera {
            // render before the "main pass" camera so we
            order: -1,
            target: image_handle.clone().into(),
            clear_color: Color::WHITE.into(),
            ..default()
        },
        // ATTENTION - THIS IS 100 BECAUSE SCENES ARE VERY ANNOYING TO BE USED WITH RENDER LAYERS
        transform: Transform::from_translation(Vec3::new(3.0, 0.0, 5.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    // The scene that oughta to be texturized
    commands.spawn(scene);

    info!("Spawning camera that it is gonna give us our wanted render");
    // Render to texture camera, renders a character
    commands.spawn(rtt_camera).insert(PanOrbitCamera::default());

    // Converting our rendered  image to an texture
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle.clone()),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

    // FORMING CUBE to see texture
    let cube_handle = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    commands.spawn((
        PbrBundle {
            mesh: cube_handle,
            material: material_handle,
            transform: Transform::from_xyz(0.0, 0.0, 1.5),
            ..default()
        },
        CubeWithRenderTexture,
    ));

    // Simple light to see stuff on both
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });

    // this shall be the main camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
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

// Debugger function in animations
pub fn disable_culling(mut commands: Commands, skinned: Query<Entity, Added<SkinnedMesh>>) {
    for entity in &skinned {
        commands.entity(entity).insert(NoFrustumCulling);
    }
}
