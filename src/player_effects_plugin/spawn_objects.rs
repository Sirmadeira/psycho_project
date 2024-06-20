use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy_rapier3d::prelude::*;

use crate::mod_char_plugin::lib::Skeleton;
use crate::player_effects_plugin::{
    Health, Limit, PdInfo, Player, PlayerGroundCollider, SidePlayer, StatePlayerCreation, Timers,
};

// Adding physical body that will move our modular character dynamically move
// Make it as the parent of the animation bones
pub fn spawn_main_rigidbody(
    mut commands: Commands,
    mod_characters: Query<(Entity, &Name), With<Skeleton>>,
    mut next_state: ResMut<NextState<StatePlayerCreation>>,
) {
    for (player_count, (player_character, scene_name)) in mod_characters.iter().enumerate() {
        // Spawning main physical body
        let main_rigidbody = (
            RigidBody::Dynamic,
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.25, 0.0),
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
            Name::new(format!("Player_{}", player_count + 1)),
            GravityScale(1.0),
            AdditionalMassProperties::Mass(10.0),
        );
        // Testing
        let second_rigidbody = (
            RigidBody::Fixed,
            SpatialBundle {
                transform: Transform::from_xyz(2.0, 0.25, 0.0),
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
            Collider::capsule_x(0.25, 0.25),
            CollisionGroups::new(Group::GROUP_10, Group::GROUP_10),
            ActiveEvents::COLLISION_EVENTS,
            TransformBundle::from(Transform::from_xyz(0.0, 0.25, 0.0)),
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

    next_state.set(StatePlayerCreation::Done)
}
