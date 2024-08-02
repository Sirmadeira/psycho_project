use crate::spawn_game_entities::lib::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// Spawns the main collider floor and a ugly mesh
pub fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = (
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(10.0, 10.0)),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
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

pub fn spawn_wall(mut commands: Commands) {
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
