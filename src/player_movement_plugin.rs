use bevy::{
    prelude::*,
    time::{Stopwatch, Timer},
};
use bevy_rapier3d::prelude::*;
use std::time::Duration;

use crate::mod_char_plugin::{link_animations::AnimationEntityLink, spawn_scenes::StateSpawnScene};
use crate::{camera_plugin::CamInfo, treat_animations_plugin::AnimationType, world_plugin::Ground};

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<StatusEffectDash>();
        app.register_type::<PdInfo>();
        app.register_type::<Timers>();
        app.register_type::<Limit>();
        app.add_event::<MovementAction>();
        app.add_systems(
            OnEnter(StateSpawnScene::Done),
            (spawn_main_rigidbody, spawn_timers_limits),
        );
        app.init_state::<StatePlayerCreation>();
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
                // Event manager
            )
                .chain()
                .run_if(in_state(StatePlayerCreation::Done)),
        );
        app.add_systems(FixedUpdate, display_events);
        app.add_systems(
            FixedUpdate,
            move_character.run_if(in_state(StatePlayerCreation::Done)),
        );
        app.add_systems(
            FixedUpdate,
            player_look_at_camera.run_if(in_state(StatePlayerCreation::Done)),
        );
    }
}

#[derive(States, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub enum StatePlayerCreation {
    #[default]
    Spawning,
    Done,
}

// Marker component - Basically the rigid body that will move the player
#[derive(Component)]
pub struct Player;

// Marker component -
#[derive(Component)]
pub struct PlayerGroundCollider;

#[derive(Event, Debug)]
pub enum MovementAction {
    // Movement direction
    Move(Vec2),
    // Dash direction
    Dash(Vec2),
    // Jump status
    Jump,
}

// Check if is on ground
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

// Check if has dashed
#[derive(Reflect, Component, Debug)]
#[component(storage = "SparseSet")]
struct StatusEffectDash {
    dash_duration: Timer,
}

// Kind of a simple pid
#[derive(Reflect, Component, Debug)]
pub struct PdInfo {
    // Proportional gain how agressive to reac
    kp: f32,
}

// Times the dash for each key
#[derive(Reflect, Component, Debug)]
struct Timers {
    up: Stopwatch,
    down: Stopwatch,
    left: Stopwatch,
    right: Stopwatch,
}

// Amount of jumps you can have
#[derive(Reflect, Component, Debug)]
struct Limit {
    jump_limit: u8,
}

impl Default for Limit {
    fn default() -> Self {
        Self { jump_limit: 2 }
    }
}

// Adding physical body that will move our modular character dynamically move
// Make it as the parent of the animation bones
fn spawn_main_rigidbody(
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
fn spawn_timers_limits(mut commands: Commands) {
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

fn keyboard_walk(
    keys: Res<ButtonInput<KeyCode>>,
    mut movement_event_writer: EventWriter<MovementAction>,
    mut animation_type_writer: EventWriter<AnimationType>,
    q_1: Query<&Transform, With<CamInfo>>,
) {
    let Ok(cam) = q_1.get_single() else { return };

    let mut direction = Vec2::ZERO;

    let mut movetype: u8 = 0;
    //forward
    if keys.pressed(KeyCode::KeyW) {
        direction.x = cam.forward().x;
        direction.y = cam.forward().z;
        movetype = 1;
    }
    // back
    if keys.pressed(KeyCode::KeyS) {
        direction.x = cam.back().x;
        direction.y = cam.back().z;
        movetype = 2;
    }
    // left
    if keys.pressed(KeyCode::KeyA) {
        direction.x = cam.left().x;
        direction.y = cam.left().z;
        movetype = 3;
    }
    // right
    if keys.pressed(KeyCode::KeyD) {
        direction.x = cam.right().x;
        direction.y = cam.right().z;
        movetype = 4;
    }
    if direction != Vec2::ZERO {
        movement_event_writer.send(MovementAction::Move(direction.normalize_or_zero()));
        animation_type_writer.send(AnimationType::MoveType(movetype));
    }
}

fn keyboard_dash(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut movement_event_writer: EventWriter<MovementAction>,
    mut animation_type_writer: EventWriter<AnimationType>,
    mut q: Query<&mut Timers>,
    q_1: Query<&Transform, With<CamInfo>>,
    q_2: Query<Entity, With<Player>>,
    q_3: Query<Has<StatusEffectDash>, With<Player>>,
) {
    let mut p_t = q.get_single_mut().unwrap();

    let cam = q_1.get_single().unwrap();

    let has_dashed = q_3.get_single().unwrap();

    let mut direction = Vec2::ZERO;

    let mut movetype: u8 = 0;

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
        movetype = 1;
    }
    if keys.just_released(KeyCode::KeyS) {
        p_t.down.reset();
    }
    if p_t.down.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyS) {
        direction.x = cam.back().x;
        direction.y = cam.back().z;
        movetype = 2;
    }
    if keys.just_released(KeyCode::KeyA) {
        p_t.left.reset();
    }
    if p_t.left.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyA) {
        direction.x = cam.left().x;
        direction.y = cam.left().z;
        movetype = 3;
    }
    if keys.just_released(KeyCode::KeyD) {
        p_t.right.reset();
    }
    if p_t.right.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyD) {
        direction.x = cam.right().x;
        direction.y = cam.right().z;
        movetype = 4;
    }

    if direction != Vec2::ZERO && has_dashed == false {
        // Add dash status effect
        let entity_1 = q_2.get_single().expect("Player to exist");
        let status_dash = StatusEffectDash {
            dash_duration: Timer::new(Duration::from_secs_f32(2.0), TimerMode::Once),
        };
        commands.entity(entity_1).insert(status_dash);

        // Sending events
        movement_event_writer.send(MovementAction::Dash(direction.normalize_or_zero()));
        animation_type_writer.send(AnimationType::MoveType(movetype));
    }
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
    q_1: Query<Entity, With<PlayerGroundCollider>>,
    q_2: Query<Entity, With<Ground>>,
) {
    // Grabs every hitbox and check if any of them are touching the ground.
    let entity1 = q_1.get_single().expect("Player to exist");
    let entity2 = q_2.get_single().expect("Ground to exist");
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
    q_1: Query<Has<Grounded>, With<PlayerGroundCollider>>,
    mut q_2: Query<&mut Limit>,
    mut movement_event_writer: EventWriter<MovementAction>,
    mut animation_type_writer: EventWriter<AnimationType>,
) {
    let is_grounded = q_1
        .get_single()
        .expect("PlayerCollider to have status grounder");

    let movetype: u8 = 5;

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
            animation_type_writer.send(AnimationType::MoveType(movetype));
        }
    }
}

fn move_character(
    mut movement_event_reader: EventReader<MovementAction>,
    time: Res<Time>,
    mut q_1: Query<(&mut Velocity, &mut ExternalImpulse), With<Player>>,
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
                MovementAction::Jump => vel.linvel.y += 15.0,
            }
        }
    }
}

// Tells me where each specific scene will look at
fn player_scenes_look_at(
){

}



fn player_look_at_camera(
    q_1: Query<&Transform, With<CamInfo>>,
    q_2: Query<(&Transform, &PdInfo), With<Player>>,
    mut q_3: Query<&mut Velocity, With<Player>>,
) {
    let cam_transform = q_1.get_single().expect("Camera to exist");
    let (player_transform, pd_info) = q_2.get_single().expect("Player to exist");

    let rot_error = (cam_transform.rotation * player_transform.rotation.inverse()).normalize();

    let (axis_error, angle_error) = rot_error.to_axis_angle();

    let angle_error_rad = angle_error.to_radians();

    let angvel = pd_info.kp * angle_error_rad * axis_error;

    for mut v in q_3.iter_mut() {
        v.angvel = angvel;
    }
}
