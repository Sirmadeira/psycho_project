use crate::ingame_camera::lib::{CamInfo, Zoom};
use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
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

    commands.spawn(camera);
}
