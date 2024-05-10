use crate::{camera_plugin::CamInfo, world_plugin::Ground};
use bevy::{
    prelude::*,
    time::{Stopwatch, Timer},
};
use bevy_rapier3d:: prelude::*;
use std::time::Duration;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementAction>();
        app.add_systems(Startup, (spawn_hitbox, spawn_others).chain());
        app.add_systems(
            Update,
            (
                // Check status effects on player
                check_status_grounded,
                check_status_effect,
                // Input handler
                keyboard_walk,
                keyboard_dash,
                keyboard_jump,
                move_character,
                apply_movement_damping,
                make_collider_look_at,
            )
                .chain(),
        );

        app.add_systems(Update, display_events);
    }
}

#[derive(Event)]
pub enum MovementAction {
    // Movement direction
    Move(Vec2),
    // Dash direction
    Dash(Vec2),
    // Jump status
    Jump,
}
// Marker component
#[derive(Component)]
pub struct Player;
// Marker component
#[derive(Component)]
pub struct PlayerRender;

// Marker component
#[derive(Component)]
pub struct Torso;

// Marker component
#[derive(Component)]
pub struct LLeg;

// Marker component
#[derive(Component)]
pub struct RLeg;

// Marker component
#[derive(Component)]
pub struct Head;

// Check if is on ground
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;
// Check if has dashed
#[derive(Component)]
#[component(storage = "SparseSet")]
struct StatusEffectDash {
    dash_duration: Timer,
}

// Times the dash for each key
#[derive(Component)]
struct Timers {
    up: Stopwatch,
    down: Stopwatch,
    left: Stopwatch,
    right: Stopwatch,
}

// Amount of jumps you can have
#[derive(Component)]
struct Limit {
    jump_limit: u8,
}

impl Default for Limit {
    fn default() -> Self {
        Self { jump_limit: 2 }
    }
}

// Spawn the hitbox and the player character
fn spawn_hitbox(mut commands: Commands) {

    // Adds all the physics to the player
    let main_rigidbody = (
        RigidBody::Dynamic,
        Player,
        AdditionalMassProperties::Mass(1.0),
        SpatialBundle {
            transform: Transform::from_xyz(0.0, 2.6, 0.0),
            ..Default::default()
        },
        Velocity::zero(),
        Damping {
            linear_damping: 0.0,
            angular_damping: 0.0,
        },
        GravityScale(1.0),
        ExternalImpulse {
            impulse: Vec3::ZERO,
            torque_impulse: Vec3::ZERO,
        },
    );

    let l_leg = (
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::round_cylinder(0.9, 0.09, 0.08),
        LLeg,
        ActiveEvents::COLLISION_EVENTS,
        CollisionGroups::new(Group::GROUP_1, Group::GROUP_1),
    );

    let r_leg = (
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::round_cylinder(0.9, 0.09, 0.08),
        RLeg,
        CollisionGroups::new(Group::GROUP_2, Group::NONE),
    );

    let torso = (
        RigidBody::Dynamic,
        Torso,
        Collider::round_cylinder(0.45, 0.18, 0.13),
        CollisionGroups::new(Group::GROUP_2, Group::NONE),
    );

    let head = (
        RigidBody::Dynamic,
        Head,
        LockedAxes::ROTATION_LOCKED,
        Collider::round_cylinder(0.25,0.15,0.10),
        CollisionGroups::new(Group::GROUP_2,Group::NONE));


    let par_entity = commands.spawn(main_rigidbody)
    .insert(torso)
    .id();

    let lleg_joint = SphericalJointBuilder::new()
    .local_anchor1(Vec3::new(0.2,-0.5,0.0))
    .local_anchor2(Vec3::new(0.0,1.1,0.0));

    let rleg_joint = SphericalJointBuilder::new()
    .local_anchor1(Vec3::new(-0.2,-0.5,0.0))
    .local_anchor2(Vec3::new(0.0, 1.1, 0.0));
    
    let head_joint = SphericalJointBuilder::new()
    .local_anchor1(Vec3::new(0.0,0.7,0.0))
    .local_anchor2(Vec3::new(0.0, -0.25, 0.0));

    commands.spawn(l_leg)
        .insert(ImpulseJoint::new(par_entity,lleg_joint));

    commands.spawn(r_leg)
        .insert(ImpulseJoint::new(par_entity,rleg_joint));

    commands.spawn(head)
        .insert(ImpulseJoint::new(par_entity,head_joint));


}

// Spawn other components
fn spawn_others(mut commands: Commands) {
    let timers = (
        Timers {
            up: Stopwatch::new(),
            down: Stopwatch::new(),
            left: Stopwatch::new(),
            right: Stopwatch::new(),
        },
        Name::new("DashTimers"),
    );

    let limit = Limit {
        ..Default::default()
    };

    commands.spawn(timers);
    commands.spawn(limit);
}

// Usefull info
fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.read() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.read() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}

fn keyboard_walk(
    keys: Res<ButtonInput<KeyCode>>,
    mut movement_event_writer: EventWriter<MovementAction>,
    q_1: Query<&Transform, With<CamInfo>>,
) {
    let Ok(cam) = q_1.get_single() else { return };

    let mut direction = Vec2::ZERO;

    //forward
    if keys.pressed(KeyCode::KeyW) {
        direction.x = cam.forward().x;
        direction.y = cam.forward().z;
    }
    // back
    if keys.pressed(KeyCode::KeyS) {
        direction.x = cam.back().x;
        direction.y = cam.back().z;
    }
    // left
    if keys.pressed(KeyCode::KeyA) {
        direction.x = cam.left().x;
        direction.y = cam.left().z;
    }
    // right
    if keys.pressed(KeyCode::KeyD) {
        direction.x = cam.right().x;
        direction.y = cam.right().z;
    }
    if direction != Vec2::ZERO {
        movement_event_writer.send(MovementAction::Move(direction.normalize_or_zero()));
    }
}

fn keyboard_dash(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut movement_event_writer: EventWriter<MovementAction>,
    mut q: Query<&mut Timers>,
    q_1: Query<&Transform, With<CamInfo>>,
    q_2: Query<(Entity, &Player)>,
    q_3: Query<Has<StatusEffectDash>, With<Player>>,
) {
    let mut p_t = q.get_single_mut().unwrap();

    let cam = q_1.get_single().unwrap();

    let has_dashed = q_3.get_single().unwrap();

    let mut direction = Vec2::ZERO;

    p_t.up.tick(Duration::from_secs_f32(time.delta_seconds()));
    p_t.down.tick(Duration::from_secs_f32(time.delta_seconds()));
    p_t.left.tick(Duration::from_secs_f32(time.delta_seconds()));
    p_t.right
        .tick(Duration::from_secs_f32(time.delta_seconds()));

    if keys.just_released(KeyCode::KeyW) {
        p_t.up.reset();
    }
    if p_t.up.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyW) {
        direction.x = cam.forward().x;
        direction.y = cam.forward().z;
    }
    if keys.just_released(KeyCode::KeyS) {
        p_t.down.reset();
    }
    if p_t.down.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyS) {
        direction.x = cam.back().x;
        direction.y = cam.back().z;
    }
    if keys.just_released(KeyCode::KeyA) {
        p_t.left.reset();
    }
    if p_t.left.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyA) {
        direction.x = cam.left().x;
        direction.y = cam.left().z;
    }
    if keys.just_released(KeyCode::KeyD) {
        p_t.right.reset();
    }
    if p_t.right.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyD) {
        direction.x = cam.right().x;
        direction.y = cam.right().z;
    }

    if direction != Vec2::ZERO && has_dashed == false {
        movement_event_writer.send(MovementAction::Dash(direction.normalize_or_zero()));
        // Add dash status effect
        let entity_1 = q_2.get_single().unwrap().0;
        let status_dash = StatusEffectDash {
            dash_duration: Timer::new(Duration::from_secs_f32(2.0), TimerMode::Once),
        };
        commands.entity(entity_1).insert(status_dash);
    }
}

fn check_status_effect(
    time: Res<Time>,
    mut commands: Commands,
    mut q_1: Query<(Entity, Option<&mut StatusEffectDash>), With<Player>>,
) {
    for (ent, status) in q_1.iter_mut() {
        if let Some(mut status) = status {
            status
                .dash_duration
                .tick(Duration::from_secs_f32(time.delta_seconds()));
            if status.dash_duration.finished() {
                commands.entity(ent).remove::<StatusEffectDash>();
            }
        } else {
            return;
        }
    }
}

pub fn check_status_grounded(
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
    q_1: Query<(Entity, &LLeg)>,
    q_2: Query<(Entity, &Ground)>,
) {
    let entity1 = q_1.get_single().unwrap().0; // A first entity with a collider attached.
    let entity2 = q_2.get_single().unwrap().0; // A second entity with a collider attached.

    /* Find the contact pair, if it exists, between two colliders. */
    if let Some(contact_pair) = rapier_context.contact_pair(entity1, entity2) {
        // The contact pair exists meaning that the broad-phase identified a potential contact.
        if contact_pair.has_any_active_contacts() {
            // The contact pair has active contacts, meaning that it
            // contains contacts for which contact forces were computed.
            commands.entity(entity1).insert(Grounded);
        }
    } else {
        commands.entity(entity1).remove::<Grounded>();
    }
}

fn keyboard_jump(
    keys: Res<ButtonInput<KeyCode>>,
    mut movement_event_writer: EventWriter<MovementAction>,
    q_1: Query<Has<Grounded>, With<LLeg>>,
    mut q_2: Query<&mut Limit>,
) {
    let is_grounded = q_1.get_single().unwrap();

    for mut jumps in q_2.iter_mut() {
        if is_grounded {
            jumps.jump_limit = Limit {
                ..Default::default()
            }
            .jump_limit
        }
        if keys.just_pressed(KeyCode::Space) && jumps.jump_limit != 0 {
            jumps.jump_limit -= 1;
            movement_event_writer.send(MovementAction::Jump);
        }
    }
}

fn move_character(
    mut movement_event_reader: EventReader<MovementAction>,
    time: Res<Time>,
    mut q_1: Query<(&mut Velocity, &mut ExternalImpulse), (With<Player>, Without<PlayerRender>)>,
) {
    for event in movement_event_reader.read() {
        for (mut vel, mut status) in &mut q_1 {
            match event {
                MovementAction::Move(direction) => {
                    vel.linvel.x += direction.x * 20.0 * time.delta_seconds();
                    vel.linvel.z += direction.y * 20.0 * time.delta_seconds();
                }
                MovementAction::Dash(direction) => {
                    status.impulse.x = direction.x * 100.0;
                    status.impulse.z = direction.y * 100.0;
                }
                MovementAction::Jump => vel.linvel.y = 15.0,
            }
        }
    }
}

fn apply_movement_damping(mut query: Query<&mut Damping, With<Player>>) {
    for mut damping_factor in &mut query {
        // Todo create state slowing down
        damping_factor.linear_damping = 0.9;
    }
}

fn make_collider_look_at(
    player_q: Query<&Transform, With<Player>>,
    cam_q: Query<&Transform, With<CamInfo>>,
    mut vel: Query<&mut Velocity>,
) {
    let mut current_time = 0.0; // Current time, starting from 0
    let total_s = 1.0; // Max s value of interpolation in seconds
    let dt = 1.0 / 60.0; // Time step for interpolation, adjust as needed

    let current_q = player_q.get_single().unwrap().rotation.normalize();
    let target_q = cam_q.get_single().unwrap().rotation.normalize();
    
    for mut v in vel.iter_mut() {
        while current_time < total_s {
            let s = current_time / total_s;

            let interpolated_q = current_q.slerp(target_q, s);

            let q_difference = interpolated_q * current_q.inverse();

            let (axis, angle) = q_difference.to_axis_angle();

            let angvel = (
                axis[0] * angle / dt,
                axis[1] * angle / dt,
                axis[2] * angle / dt,
            );

            v.angvel = angvel.into();

            current_time += dt;
        }
    }
}
