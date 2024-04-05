use bevy::{
    input::mouse::{MouseMotion,MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};

use std::f32::consts::PI;

use iyes_perf_ui::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, (orbit_mouse,zoom_mouse).chain());
    }
}

// Identify our camera
#[derive(Component)]
pub struct CamInfo {
    mouse_sens: f32,
    zoom_sens: f32,
    zoom: Zoom
}

/// Sets the zoom bounds (min & max)
pub struct Zoom {
    pub min: f32,
    pub max: f32,
    radius: f32,
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


fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
            projection: PerspectiveProjection {
                fov:45.0_f32.to_radians(),
                ..default()
            }.into(),
            ..default()
        },
        CamInfo{
            mouse_sens: 10.0,
            zoom_sens: 2.0,
            zoom: Zoom::new(10.0,30.0)
        }
    );

    commands.spawn(camera);
    commands.spawn(PerfUiCompleteBundle::default());
}


pub fn orbit_mouse(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut cam_q: Query<(&CamInfo,&mut Transform), With<CamInfo>>,
    mut mouse_evr: EventReader<MouseMotion>,)
    {

    // Basing the rotation according to the mouve motion
    let mut rotation = Vec2::ZERO;
    for ev in mouse_evr.read() {
            rotation = ev.delta;
        }
    
    let (cam_info,mut cam_transform) = cam_q.get_single_mut().unwrap();


    if rotation.length_squared() > 0.0 {
        let window = window_q.get_single().unwrap();
        let delta_x = {
            let delta = rotation.x / window.width() * std::f32::consts::PI * cam_info.mouse_sens;
            delta
        };

        let delta_y = rotation.y / window.height() * PI * cam_info.mouse_sens;
        let yaw = Quat::from_rotation_y(-delta_x);
        let pitch = Quat::from_rotation_x(-delta_y);
        cam_transform.rotation = yaw * cam_transform.rotation; // rotate around global y axis

        // Calculate the new rotation without applying it to the camera yet
        let new_rotation = cam_transform.rotation * pitch;

        // check if new rotation will cause camera to go beyond the 180 degree vertical bounds
        let up_vector = new_rotation * Vec3::Y;
        if up_vector.y > 0.0 {
            cam_transform.rotation = new_rotation;
        }
    }

    let rot_matrix = Mat3::from_quat(cam_transform.rotation);
    cam_transform.translation = rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam_info.zoom.radius));


}


fn zoom_mouse(mut scroll_evr: EventReader<MouseWheel>, mut cam_q: Query<&mut CamInfo>) {
    let mut scroll = 0.0;
    for ev in scroll_evr.read() {
        scroll += ev.y;
    }

    if let Ok(mut cam) = cam_q.get_single_mut() {
        if scroll.abs() > 0.0 {
            let new_radius =
                cam.zoom.radius - scroll * cam.zoom.radius * 0.1 * cam.zoom_sens;
            cam.zoom.radius = new_radius.clamp(cam.zoom.min, cam.zoom.max);
        }
    }
}