use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

use crate::MyAppState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(MyAppState::InGame),
            (spawn_light, spawn_floor, spawn_wall),
        )
        .add_systems(Update, animate_light_direction);
    }
}

// Marks ground entities
#[derive(Component)]
pub struct Ground;
// Marks wall entities
#[derive(Component)]
pub struct Wall;

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

// Spawns the main collider floor and a ugly mesh
fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = (
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(10.0, 10.0)),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            ..default()
        },
        Name::new("Floor"),
    );
    // He is group 10 because for now we can only have 10 players
    let collider = (
        Collider::cuboid(100.0, 0.5, 100.0),
        Ground,
        CollisionGroups::new(Group::GROUP_10, Group::ALL),
    );

    commands.spawn(floor).insert(collider);
}

fn spawn_wall(mut commands: Commands) {
    let wall_collider = (
        Collider::cuboid(1.0, 10.0, 10.0),
        CollisionGroups::new(Group::GROUP_10, Group::ALL),
        Wall,
        ActiveEvents::COLLISION_EVENTS,
    );

    commands
        .spawn(RigidBody::Fixed)
        .insert(SpatialBundle {
            transform: Transform::from_xyz(10.0, 10.0, 10.0),
            ..Default::default()
        })
        .insert(Name::new("Wall"))
        .with_children(|children| {
            children.spawn(wall_collider);
        });
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() * 0.5);
    }
}
