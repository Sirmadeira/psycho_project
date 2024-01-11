use bevy::prelude::*;

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_gun);
    }
}

fn spawn_gun(mut commands: Commands, assets: Res<AssetServer>) {
    let sword = SceneBundle {
        scene: assets.load("sword.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.5, 0.0),
            rotation: Quat::from_axis_angle(Vec3::new(1.0, 0.0, 0.0), 1.55),
            ..default()
        },
        ..default()
    };

    commands.spawn(sword);
}
