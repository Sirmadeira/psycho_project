use bevy::prelude::*;
use iyes_perf_ui::prelude::*;

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
    );

    commands.spawn(camera);
    commands.spawn(PerfUiCompleteBundle::default());
}
