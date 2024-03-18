use std::f32::consts::PI;
use bevy::{
    pbr:: CascadeShadowConfigBuilder,
    prelude::*,
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_light, spawn_floor))
        .add_systems(Update,animate_light_direction);
    }
}

fn spawn_light(mut commands: Commands) {
    let sun_light = DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    };  
    commands.spawn(sun_light);
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // O chao de tudo
    let floor = (
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            ..default()
        },
        Name::new("Floor"),
    );
    commands.spawn(floor);
    }

fn animate_light_direction(
        time: Res<Time>,
        mut query: Query<&mut Transform, With<DirectionalLight>>,
    ) {
        for mut transform in &mut query {
            transform.rotate_y(time.delta_seconds() * 0.5);
        }
    }
    
