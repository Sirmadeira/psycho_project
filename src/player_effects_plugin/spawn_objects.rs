use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy_rapier3d::prelude::*;

use crate::mod_char_plugin::link_animations::AnimationEntityLink;
use crate::player_effects_plugin::{
    Limit, PdInfo, Player, PlayerGroundCollider, StatePlayerCreation, Timers,
};

// Adding physical body that will move our modular character dynamically move
// Make it as the parent of the animation bones
pub fn spawn_main_rigidbody(
    mut commands: Commands,
    mod_character: Query<Entity, With<AnimationEntityLink>>,
    mut next_state: ResMut<NextState<StatePlayerCreation>>,
) {
    // Getting modular character
    let character = mod_character
        .get_single()
        .expect("Modular character to exist");
    // Spawning main physical body
    let main_rigidbody = (
        RigidBody::Dynamic,
        Player,
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
        Name::new("Player1"),
        GravityScale(1.0),
        AdditionalMassProperties::Mass(10.0),
    );
    // Spawning the collider that detects collision
    let main_collider = (
        Collider::capsule_x(0.25, 0.25),
        CollisionGroups::new(Group::GROUP_1, Group::GROUP_1),
        ActiveEvents::COLLISION_EVENTS,
        TransformBundle::from(Transform::from_xyz(0.0, 0.25, 0.0)),
        PlayerGroundCollider,
    );

    let main_rigidbody_entity_id = commands
        .spawn(main_rigidbody)
        .with_children(|children| {
            children.spawn(main_collider);
        })
        .id();

    commands
        .entity(character)
        .set_parent(main_rigidbody_entity_id);

    next_state.set(StatePlayerCreation::Done)
}

// Spawn other components
pub fn spawn_timers_limits(mut commands: Commands) {
    let timers = (
        Timers {
            up: Stopwatch::new(),
            down: Stopwatch::new(),
            left: Stopwatch::new(),
            right: Stopwatch::new(),
        },
        Name::new("DashTimers"),
    );

    let limit = (
        Limit {
            ..Default::default()
        },
        Name::new("Limits"),
    );

    commands.spawn(timers);
    commands.spawn(limit);
}
