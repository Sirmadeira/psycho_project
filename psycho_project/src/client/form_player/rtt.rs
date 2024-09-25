//! Just a separate sub-mod to handle the rtt camera asset maker
//! Since it is used in lobby_screen I dissociated for now
//! Worth noting most of it is logic is still very co dependent with customize character
use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::window::PrimaryWindow;
use bevy_panorbit_camera::{ActiveCameraData, PanOrbitCamera};

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