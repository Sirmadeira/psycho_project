use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy_atmosphere::plugin::AtmosphereCamera;
use std::f32::consts::PI;


// Camera
// Info for camera mechanics
#[derive(Reflect, Component, Debug)]
pub struct CamInfo {
    pub mouse_sens: f32,
    pub zoom_enabled: bool,
    pub zoom: Zoom,
    pub zoom_sens: f32,
    pub cursor_lock_activation_key: KeyCode,
    pub cursor_lock_active: bool,
}

// Sets the zoom bounds (min & max)
#[derive(Reflect, Component, Debug)]
pub struct Zoom {
    pub min: f32,
    pub max: f32,
    pub radius: f32,
}

impl Zoom {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min,
            max,
            radius: (min + max) / 2.0,
        }
    }
}

// Atmospher/Lighting

// Marker component
#[derive(Component)]
pub struct Sun;

// Timer to update it is gonna be a biggie while i am debuggin
#[derive(Resource)]
pub struct CycleTimer(pub Timer);

pub fn spawn_camera_atmosphere(mut commands: Commands) {
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 3.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CamInfo {
            mouse_sens: 0.75,
            zoom_enabled: true,
            zoom: Zoom::new(5.0, 20.0),
            zoom_sens: 2.0,
            cursor_lock_activation_key: KeyCode::KeyE,
            cursor_lock_active: false,
        },
    );

    let sun_light = DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    };

    // Atmosphere shading is heavily associated with camera
    commands.spawn(camera).insert(AtmosphereCamera::default());
    commands.spawn(sun_light).insert(Sun);
}
