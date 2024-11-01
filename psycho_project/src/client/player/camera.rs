//! Super camera is gonna have orbit mode and some following shit like my old one
//! YEAH
use crate::client::MyAppState;
use bevy::input::mouse::MouseMotion;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use core::f32::consts::PI;
use lightyear::client::prediction::Predicted;
use lightyear::shared::replication::components::Controlled;

use bevy_panorbit_camera::PanOrbitCamera;

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        // Debugging
        app.register_type::<Zoom>();
        app.register_type::<CamInfo>();

        app.add_systems(Startup, spawn_begin_camera);

        app.add_systems(
            Update,
            (
                toggle_cursor,
                orbit_mouse.run_if(orbit_condition),
                zoom_mouse.run_if(zoom_condition),
            )
                .run_if(in_state(MyAppState::Game))
                .chain(),
        );

        app.add_systems(Update, sync_rtt_to_player);

        app.add_systems(
            PostUpdate,
            sync_player_camera.before(TransformSystem::TransformPropagate),
        );
    }
}

// Conditions
// only run the orbit system if the cursor lock is disabled
fn orbit_condition(cam_q: Query<&CamInfo>) -> bool {
    let Ok(cam) = cam_q.get_single() else {
        return false;
    };
    return cam.cursor_lock_active;
}

// only zoom if zoom is enabled & the cursor lock feature is enabled & active
fn zoom_condition(cam_q: Query<&CamInfo, With<CamInfo>>) -> bool {
    let Ok(cam) = cam_q.get_single() else {
        return false;
    };
    return cam.zoom_enabled && cam.cursor_lock_active;
}

/// Marker component tells me who is my main camera - A lot of mechanic in the future gonna be based on it
#[derive(Component)]
pub struct MainCamera;

/// Info for camera mechanics
#[derive(Reflect, Component, Debug)]
pub struct CamInfo {
    pub mouse_sens: f32,
    pub zoom_enabled: bool,
    pub zoom: Zoom,
    pub zoom_sens: f32,
    pub cursor_lock_activation_key: KeyCode,
    pub cursor_lock_active: bool,
}

/// Sets the zoom bounds (min & max)
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

fn spawn_begin_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle::default())
        .insert(MainCamera)
        .insert(CamInfo {
            mouse_sens: 0.75,
            zoom_enabled: true,
            zoom: Zoom::new(5.0, 10.0),
            zoom_sens: 2.0,
            cursor_lock_activation_key: KeyCode::KeyR,
            cursor_lock_active: false,
        });
}

/// Turns on the ability to control the camera
fn toggle_cursor(
    mut cam_q: Query<&mut CamInfo>,
    keys: Res<ButtonInput<KeyCode>>,
    mut window_q: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(mut cam) = cam_q.get_single_mut() else {
        return;
    };

    if keys.just_pressed(cam.cursor_lock_activation_key) {
        cam.cursor_lock_active = !cam.cursor_lock_active;
    }

    let mut window = window_q.get_single_mut().unwrap();
    if cam.cursor_lock_active {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    } else {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}

/// Adds the possibility to adjust camera angle and position
fn orbit_mouse(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut cam_q: Query<(&CamInfo, &mut Transform), With<CamInfo>>,
    mut mouse_evr: EventReader<MouseMotion>,
) {
    // Basing the rotation according to the mouve motion
    let mut rotation = Vec2::ZERO;
    for ev in mouse_evr.read() {
        rotation += ev.delta;
    }

    let (cam_info, mut cam_transform) = cam_q.get_single_mut().unwrap();

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

        // check if new rotation will cause camera to go beyond the 180 degree vertical boundse
        let up_vector = new_rotation * Vec3::Y;
        if up_vector.y > 0.0 {
            cam_transform.rotation = new_rotation;
        }
    }

    let rot_matrix = Mat3::from_quat(cam_transform.rotation);
    cam_transform.translation = rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam_info.zoom.radius));
}

/// Zooms in the camera
fn zoom_mouse(mut scroll_evr: EventReader<MouseWheel>, mut cam_q: Query<&mut CamInfo>) {
    let mut scroll = 0.0;
    for ev in scroll_evr.read() {
        scroll += ev.y;
    }

    if let Ok(mut cam) = cam_q.get_single_mut() {
        if scroll.abs() > 0.0 {
            let new_radius = cam.zoom.radius - scroll * cam.zoom.radius * 0.1 * cam.zoom_sens;
            cam.zoom.radius = new_radius.clamp(cam.zoom.min, cam.zoom.max);
        }
    }
}

pub fn sync_player_camera(
    player_q: Query<&Transform, (With<Predicted>, With<Controlled>)>,
    mut cam_q: Query<(&mut CamInfo, &mut Transform), Without<Predicted>>,
) {
    if let Ok(player_transform) = player_q.get_single() {
        let (cam, mut cam_transform) = cam_q.get_single_mut().expect("Camera to exist");

        let rotation_matrix = Mat3::from_quat(cam_transform.rotation);

        // Offset actually
        let offset = rotation_matrix.mul_vec3(Vec3::new(0.0, 0.5, cam.zoom.radius));

        // Update the camera translation
        cam_transform.translation = offset + player_transform.translation;
    }
}

// Gonna grab controlled entity position and mark it
fn sync_rtt_to_player(
    mut pan_orbit: Query<&mut PanOrbitCamera>,
    player: Query<&Transform, (With<Predicted>, With<Controlled>)>,
) {
    for mut pan_orbit in pan_orbit.iter_mut() {
        if let Ok(target_transform) = player.get_single() {
            pan_orbit.target_focus = target_transform.translation;
            // Whenever changing properties manually like this, it's necessary to force
            // PanOrbitCamera to update this frame (by default it only updates when there are
            // input events).
            pan_orbit.force_update = true;
        }
    }
}
