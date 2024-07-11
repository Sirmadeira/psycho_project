use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy_rapier3d::prelude::*;

use crate::mod_char::lib::Skeleton;
use crate::player_effects::{
    Health, Limit, PdInfo, Player, PlayerGroundCollider, SidePlayer, Timers,
};

// Adding physical body that will move our modular character dynamically move
// Make it as the parent of the animation bones
pub fn spawn_main_rigidbody(
    mut commands: Commands,
    mod_characters: Query<(Entity, &Name), With<Skeleton>>,
) {
    for ((player_character, scene_name), player_count) in mod_characters.iter().zip(1..) {
        // Spawning main physical body
        let main_rigidbody = (
            RigidBody::Dynamic,
            SpatialBundle {
                transform: Transform::from_xyz(-2.0, 0.25, -3.0),
                ..Default::default()
            },
            Velocity::zero(),
            Damping {
                linear_damping: 0.9,
                angular_damping: 0.0,
            },
            ExternalImpulse {
                impulse: Vec3::ZERO,
                torque_impulse: Vec3::ZERO,
            },
            PdInfo { kp: 500.0 },
            Name::new(format!("Player_{}", player_count)),
            GravityScale(1.0),
            AdditionalMassProperties::Mass(10.0),
        );
        // Testing
        let second_rigidbody = (
            RigidBody::Fixed,
            SpatialBundle {
                transform: Transform::from_xyz(4.0, 0.25, 3.0),
                ..Default::default()
            },
            Velocity::zero(),
            Damping {
                linear_damping: 0.9,
                angular_damping: 0.0,
            },
            ExternalImpulse {
                impulse: Vec3::ZERO,
                torque_impulse: Vec3::ZERO,
            },
            PdInfo { kp: 500.0 },
            Name::new(format!("Player_{}", player_count)),
            GravityScale(1.0),
            AdditionalMassProperties::Mass(10.0),
        );

        // Spawning the collider that moves and tells things
        let main_collider = (
            Collider::cuboid(0.65, 1.0, 0.25),
            CollisionGroups::new(Group::GROUP_11, Group::GROUP_10),
            ActiveEvents::COLLISION_EVENTS,
            TransformBundle::from(Transform::from_xyz(0.0, 1.0, 0.0)),
            PlayerGroundCollider,
        );

        let health = Health(10);

        let timers = (Timers {
            up: Stopwatch::new(),
            down: Stopwatch::new(),
            left: Stopwatch::new(),
            right: Stopwatch::new(),
        },);

        let limit = (Limit {
            ..Default::default()
        },);

        if scene_name.to_string() == "skeleton_1" {
            let player_details = commands
                .spawn(main_rigidbody)
                .insert(Player)
                .insert(timers)
                .insert(limit)
                .insert(health)
                .with_children(|children: &mut ChildBuilder| {
                    children.spawn(main_collider);
                })
                .id();

            commands.entity(player_character).set_parent(player_details);
        } else {
            let other_details = commands
                .spawn(second_rigidbody)
                .insert(SidePlayer)
                .insert(timers)
                .insert(limit)
                .insert(health)
                .with_children(|children: &mut ChildBuilder| {
                    children.spawn(main_collider);
                })
                .id();

            commands.entity(player_character).set_parent(other_details);
        }
    }
}