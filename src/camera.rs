use bevy::prelude::*;
use bevy_third_person_camera::{camera::*,*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
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
        ThirdPersonCamera {
            zoom_enabled: true,
            zoom:Zoom::new(5.0,15.0),
            cursor_lock_toggle_enabled: true,
            cursor_lock_key: KeyCode::KeyE,
            ..default()
        },
    );

    commands.spawn(camera);
}
