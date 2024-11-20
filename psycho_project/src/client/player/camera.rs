//! Super camera is gonna have orbit mode and some following shit like my old one
//! YEAH
use crate::client::MyAppState;
use avian3d::prelude::PhysicsSet;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_panorbit_camera::PanOrbitCamera;
use core::f32::consts::PI;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::Actionlike;
use lightyear::client::prediction::Predicted;
use lightyear::shared::replication::components::Controlled;
use serde::{Deserialize, Serialize};

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        // Debugging
        app.register_type::<Zoom>();
        app.register_type::<CamInfo>();

        // This might cause issue later
        app.add_plugins(InputManagerPlugin::<CameraMovement>::default());

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

        app.add_systems(
            PostUpdate,
            sync_rtt_to_player
                .after(PhysicsSet::Sync)
                .before(TransformSystem::TransformPropagate),
        );

        app.add_systems(
            PostUpdate,
            sync_player_camera
                .after(PhysicsSet::Sync)
                .before(TransformSystem::TransformPropagate),
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
pub struct MarkerMainCamera;

/// Info for camera mechanics
#[derive(Reflect, Component, Debug)]
pub struct CamInfo {
    pub mouse_sens: f32,
    pub zoom_enabled: bool,
    pub zoom: Zoom,
    pub zoom_sens: f32,
    pub cursor_lock_activation_key: KeyCode,
    pub cursor_lock_active: bool,
    pub yaw_limit: Option<(f32, f32)>,
    pub pitch_limit: Option<(f32, f32)>,
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

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Hash, Reflect)]
enum CameraMovement {
    #[actionlike(Axis)]
    Zoom,
    #[actionlike(DualAxis)]
    Pan,
}

fn spawn_begin_camera(mut commands: Commands) {
    let input_map = InputMap::default()
        .with_dual_axis(CameraMovement::Pan, MouseMove::default())
        .with_axis(CameraMovement::Zoom, MouseScrollAxis::Y);

    commands
        .spawn(Camera3dBundle::default())
        .insert(MarkerMainCamera)
        .insert(Name::new("MainCamera"))
        .insert(CamInfo {
            mouse_sens: 0.75,
            zoom_enabled: true,
            zoom: Zoom::new(5.0, 10.0),
            zoom_sens: 2.0,
            cursor_lock_activation_key: KeyCode::KeyR,
            cursor_lock_active: false,
            yaw_limit: None,
            pitch_limit: Some((-PI / 2.0, PI / 20.0)),
        })
        .insert(InputManagerBundle::with_map(input_map));
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
    mut cam_q: Query<
        (&CamInfo, &ActionState<CameraMovement>, &mut Transform),
        With<MarkerMainCamera>,
    >,
) {
    if let (Ok(window), Ok((cam_info, camera_movement, mut cam_transform))) =
        (window_q.get_single(), cam_q.get_single_mut())
    {
        // Accumulate mouse motion into a rotation vector
        let rotation_delta: Vec2 = camera_movement.axis_pair(&CameraMovement::Pan);

        if rotation_delta.length_squared() > 0.0 {
            // Calculate normalized rotation deltas
            let delta_x = (rotation_delta.x / window.width()) * PI * cam_info.mouse_sens;
            let delta_y = (rotation_delta.y / window.height()) * PI * cam_info.mouse_sens;

            // Retrieve current yaw and pitch
            let (yaw, pitch, _) = cam_transform.rotation.to_euler(EulerRot::YXZ);

            // Apply yaw limit if set
            let new_yaw = if let Some((min_yaw, max_yaw)) = cam_info.yaw_limit {
                (yaw - delta_x).clamp(min_yaw, max_yaw)
            } else {
                yaw - delta_x
            };

            // Apply pitch limit if set
            let new_pitch = if let Some((min_pitch, max_pitch)) = cam_info.pitch_limit {
                (pitch - delta_y).clamp(min_pitch, max_pitch)
            } else {
                pitch - delta_y
            };

            // Apply rotation after limit set
            cam_transform.rotation = Quat::from_euler(EulerRot::YXZ, new_yaw, new_pitch, 0.0);

            // Update camera translation based on zoom radius
            let rot_matrix = Mat3::from_quat(cam_transform.rotation);
            cam_transform.translation =
                rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam_info.zoom.radius));
        }
    }
}

/// Zooms in the camera
fn zoom_mouse(
    mut cam_q: Query<(&mut CamInfo, &ActionState<CameraMovement>), With<MarkerMainCamera>>,
) {
    if let Ok((mut cam, camera_movement)) = cam_q.get_single_mut() {
        let scroll = camera_movement.value(&CameraMovement::Zoom);
        if scroll.abs() > 0.0 {
            let new_radius = cam.zoom.radius - scroll * cam.zoom.radius * 0.1 * cam.zoom_sens;
            cam.zoom.radius = new_radius.clamp(cam.zoom.min, cam.zoom.max);
        }
    }
}

fn sync_player_camera(
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
