use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_rapier3d::prelude::*;

use crate::camera_plugin::CamInfo;
use crate::player_effects_plugin::{
    Grounded, Limit, MovementAction, PdInfo, Player, PlayerGroundCollider, StatusEffectDash, Timers,
};
use crate::treat_animations_plugin::AnimationType;

pub fn keyboard_walk(
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

pub fn keyboard_dash(
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

pub fn keyboard_jump(
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

pub fn move_character(
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

pub fn player_look_at_camera(
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
