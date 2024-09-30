use super::load_assets::ClientCharCollection;
use super::MyAppState;
use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use bevy_panorbit_camera::{ActiveCameraData, PanOrbitCamera};

#[derive(Resource, Default)]
pub struct RttImages(pub HashMap<String, ImageInfo>);

pub struct ImageInfo {
    pub handle: Handle<Image>,
    pub size: Extent3d,
    pub scene_location: Vec3,
}

impl ImageInfo {
    fn new(handle: Handle<Image>, size: Extent3d, scene_location: Vec3) -> Self {
        return Self {
            handle: handle,
            size: size,
            scene_location: scene_location,
        };
    }
}

pub struct FormRttsPlugin;

impl Plugin for FormRttsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MyAppState::MainMenu), spawn_empty_images);

        // Rtt system
        // app.add_systems(OnEnter(MyAppState::MainMenu), form_rtts_for_assets);
    }
}

// Creates a bunch of empty images according to the amount of gltf_files  also spawn their given
fn spawn_empty_images(
    client_collection: Res<ClientCharCollection>,
    mut images: ResMut<Assets<Image>>,
    assets_gltf: Res<Assets<Gltf>>,
    mut commands: Commands,
) {
    let mut rtt_images = HashMap::new();
    // All assets
    let gltfs = &client_collection.gltf_files;
    // Image res
    let size = Extent3d {
        width: 4096,
        height: 4096,
        ..default()
    };
    // Details
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

    // Defining transform - Putting really far aways cuz we dont players to see this
    let mut scene_location = Vec3::new(20.0, 1.0, 0.0);
    let mut image_info;
    for (name, gltf) in gltfs.iter() {
        info_once!("Spawning scene {} for RTT", name);
        let asset = assets_gltf.get(gltf).expect("To be able to grab it");
        let visual_scene = asset.scenes[0].clone();
        let scene = SceneBundle {
            scene: visual_scene,
            transform: Transform::from_translation(scene_location)
                // If you want him to stare face front to camera as from blender he usually stares at negative -z
                .looking_at(Vec3::new(0.0, 0.0, 1.0), Vec3::Y),
            ..default()
        };
        commands.spawn(scene);
        scene_location.x += 5.0;

        // Forming resource
        let image_handle = images.add(image.clone());

        image_info = ImageInfo::new(image_handle, size, scene_location);

        rtt_images.insert(name.to_string(), image_info);
    }

    commands.insert_resource(RttImages(rtt_images));
}

pub fn spawn_rtt_orbit_camera(
    rtt_image_info: &ImageInfo,
    camera_offset: Vec3,
    windows: &Query<&Window, With<PrimaryWindow>>,
    active_cam: &mut ResMut<ActiveCameraData>,
    commands: &mut Commands,
) {
    // let mut camera_offset = Vec3::new(0.0, 1.5, 3.5);
    let rtt_camera = Camera3dBundle {
        camera: Camera {
            // render before the "main pass" camera so we
            order: -1,
            target: rtt_image_info.handle.clone().into(),
            clear_color: ClearColorConfig::Custom(Color::srgb(0.212, 0.271, 0.31)),
            ..default()
        },
        transform: Transform::from_translation(camera_offset),
        ..default()
    };
    // Important component to let player control
    let pan_orbit = PanOrbitCamera {
        focus: rtt_image_info.scene_location,
        zoom_upper_limit: Some(3.5),
        zoom_lower_limit: Some(1.0),
        ..default()
    };

    //Spawning camera that it is gonna give us our wanted render
    // Render to texture camera, renders a character
    let pan_orbit_id = commands.spawn(rtt_camera).insert(pan_orbit).id();

    let primary_window = windows
        .get_single()
        .expect("There is only ever one primary window");

    active_cam.set_if_neq(ActiveCameraData {
        entity: Some(pan_orbit_id),
        viewport_size: Some(Vec2::new(
            rtt_image_info.size.width as f32,
            rtt_image_info.size.height as f32,
        )),
        window_size: Some(Vec2::new(primary_window.width(), primary_window.height())),
        // Setting manual to true ensures PanOrbitCameraPlugin will not overwrite this resource
        manual: true,
    });
}
