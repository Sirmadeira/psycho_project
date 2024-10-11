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
}

impl ImageInfo {
    fn new(handle: Handle<Image>, size: Extent3d) -> Self {
        return Self {
            handle: handle,
            size: size,
        };
    }
}

pub struct FormRttsPlugin;

impl Plugin for FormRttsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_empty_images);
    }
}

// If you want to create an rtt just use this guy and add the scenario you want to orbit around
fn spawn_empty_images(mut images: ResMut<Assets<Image>>, mut commands: Commands) {
    let mut rtt_images = HashMap::new();
    // All assets
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
    let image_info = ImageInfo::new(images.add(image.clone()), size);

    rtt_images.insert("Character".to_string(), image_info);

    commands.insert_resource(RttImages(rtt_images));
}

pub fn spawn_rtt_orbit_camera(
    rtt_image_info: &ImageInfo,
    windows: &Query<&Window, With<PrimaryWindow>>,
    active_cam: &mut ResMut<ActiveCameraData>,
    commands: &mut Commands,
) {
    let rtt_camera = Camera3dBundle {
        camera: Camera {
            // render before the "main pass" camera so we
            order: -1,
            target: rtt_image_info.handle.clone().into(),
            clear_color: ClearColorConfig::Custom(Color::srgb(0.212, 0.271, 0.31)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 1.5, 3.5)),
        ..default()
    };
    // Important component to let player control
    let pan_orbit = PanOrbitCamera {
        focus: Vec3::new(0.0, 1.0, 0.0),
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
