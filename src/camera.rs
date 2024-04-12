use bevy::{
    input::mouse::{MouseMotion,MouseWheel}, prelude::*, window::{CursorGrabMode, PrimaryWindow}
};
use bevy_rapier3d::plugin::PhysicsSet;

use crate::player:: Player;
use std::f32::consts::PI;
use iyes_perf_ui::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, 
        (toggle_cursor,
        orbit_mouse.run_if(orbit_condition),
        zoom_mouse.run_if(zoom_condition))
        .chain());
        app.add_systems(PostUpdate, sync_player_camera.after(PhysicsSet::StepSimulation));
    }
}

// Setting of my camera
#[derive(Component)]
pub struct CamInfo {
    mouse_sens: f32,
    zoom_enabled: bool,
    zoom: Zoom,
    zoom_sens: f32,
    cursor_lock_activation_key:KeyCode,
    cursor_lock_active: bool,

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
            ..default()
        },
        CamInfo{
            mouse_sens: 1.0,
            zoom_enabled: true,
            zoom: Zoom::new(10.0,30.0),
            zoom_sens: 2.0,
            cursor_lock_activation_key: KeyCode::KeyE,
            cursor_lock_active:false,
        }
    );

    commands.spawn(camera);
    commands.spawn(PerfUiCompleteBundle::default());
}

// Turns on the ability to control the camera
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


// Adds the possibility to adjust camera angle and position
fn orbit_mouse(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut cam_q: Query<(&CamInfo,&mut Transform), With<CamInfo>>,
    mut mouse_evr: EventReader<MouseMotion>,)
    {

    // Basing the rotation according to the mouve motion
    let mut rotation = Vec2::ZERO;
    for ev in mouse_evr.read() {
            rotation += ev.delta;
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

// Zooms in the camera
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



fn sync_player_camera(
    player_q: Query<&Transform, With<Player>>,
    mut cam_q: Query<(&mut CamInfo, &mut Transform),Without<Player>>,
) {
    let Ok(player) = player_q.get_single() else {
        return;
    };
    let Ok((cam, mut cam_transform)) = cam_q.get_single_mut() else {
        return;
    };

    let rotation_matrix = Mat3::from_quat(cam_transform.rotation);


    let desired_translation = rotation_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam.zoom.radius)); 
    // Update the camera translation
    cam_transform.translation = desired_translation + player.translation;
}


// Conditions
// only run the orbit system if the cursor lock is disabled
fn orbit_condition(cam_q: Query<&CamInfo>) -> bool {
    let Ok(cam) = cam_q.get_single() else {
        return true;
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