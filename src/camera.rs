use bevy::prelude::*;
use bevy_third_person_camera::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ThirdPersonCamera {
            aim_enabled: true,
            aim_zoom: 0.1,
            zoom_enabled: true,
            zoom: Zoom::new(1.5, 5.0),
            ..default()
        },
    );

    commands.spawn(camera);
}
