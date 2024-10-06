//! Super camera is gonna have orbit mode and some following shit like my old one
//! YEAH
use bevy::prelude::*;

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_begin_camera);
    }
}

// Marker component tells me who is my main camera - A lot of mechanic in the future gonna be based on it
#[derive(Component)]
pub struct MainCamera;

fn spawn_begin_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 4.5))
                .looking_at(Vec3::new(0.0, 1.5, 0.0), Vec3::Y),
            ..default()
        })
        .insert(MainCamera);
}
