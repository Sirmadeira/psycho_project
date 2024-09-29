use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use bevy_panorbit_camera::{ActiveCameraData, PanOrbitCamera};

use super::load_assets::ClientCharCollection;
use super::MyAppState;

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct RttImages(pub HashMap<String, Handle<Image>>);

pub struct FormRttsPlugin;

impl Plugin for FormRttsPlugin {
    fn build(&self, app: &mut App) {
        // Debugging
        app.register_type::<RttImages>();

        // Rtt system
        app.add_systems(OnEnter(MyAppState::MainMenu), form_rtts_for_assets);
    }
}

// This will make an rtt with an available pan orbit camera pointing at it. And saves the asset  as an material
pub fn spawn_rtt_camera(
    windows: &Query<&Window, With<PrimaryWindow>>,
    images: &mut ResMut<Assets<Image>>,
    active_cam: &mut ResMut<ActiveCameraData>,
    commands: &mut Commands,
    camera_offset: Vec3,
) -> Handle<Image> {
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
    return image_handle;
}

fn form_rtts_for_assets(
    client_collection: Res<ClientCharCollection>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut images: ResMut<Assets<Image>>,
    mut assets_gltf: ResMut<Assets<Gltf>>,
    mut active_cam: ResMut<ActiveCameraData>,
    mut commands: Commands,
) {
    let gltfs = &client_collection.gltf_files;
    let mut rtt_images = HashMap::new();

    info!("This sets up the offset");
    let mut offset = Vec3::new(0.0, 1.0, 0.0);
    let mut camera_offset = Vec3::new(0.0, 1.5, 3.5);

    for (name, gltf) in gltfs.iter() {
        info_once!("Spawning camera");
        let handle = spawn_rtt_camera(
            &windows,
            &mut images,
            &mut active_cam,
            &mut commands,
            camera_offset,
        );
        info_once!("Spawning scene with specific transform");
        let asset = assets_gltf.get(gltf).expect("To be able to grab it");
        let visual_scene = asset.scenes[0].clone();
        let scene = SceneBundle {
            scene: visual_scene,
            transform: Transform::from_translation(offset),
            // If you want him to stare face front to camera as from blender he usually stares at negative -z
            // .looking_at(Vec3::new(0.0, 0.0, 1.0), Vec3::Y),
            ..default()
        };
        commands.spawn(scene);
        info_once!("Offsetting so they dont overlap");
        offset.x += 0.5;
        camera_offset.x += 0.5;
        info_once!("Insert in resource");
        rtt_images.insert(name.to_string(), handle);
    }
    commands.insert_resource(RttImages(rtt_images));
}
