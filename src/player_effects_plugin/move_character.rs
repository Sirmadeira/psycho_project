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
    mut q: Query<(&mut Timers, Entity, Has<StatusEffectDash>), With<Player>>,
    q_1: Query<&Transform, With<CamInfo>>,
) {
    for (mut timers, player, has_dashed) in q.iter_mut() {
        let mut movetype: u8 = 0;
        let mut direction = Vec2::ZERO;
        let cam = q_1.get_single().unwrap();

        timers
            .up
            .tick(Duration::from_secs_f32(time.delta_seconds()));
        timers
            .down
            .tick(Duration::from_secs_f32(time.delta_seconds()));
        timers
            .left
            .tick(Duration::from_secs_f32(time.delta_seconds()));
        timers
            .right
            .tick(Duration::from_secs_f32(time.delta_seconds()));

        if keys.just_released(KeyCode::KeyW) {
            timers.up.reset();
        }
        if timers.up.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyW) {
            direction.x = cam.forward().x;
            direction.y = cam.forward().z;
            movetype = 1;
        }
        if keys.just_released(KeyCode::KeyS) {
            timers.down.reset();
        }
        if timers.down.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyS) {
            direction.x = cam.back().x;
            direction.y = cam.back().z;
            movetype = 2;
        }
        if keys.just_released(KeyCode::KeyA) {
            timers.left.reset();
        }
        if timers.left.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyA) {
            direction.x = cam.left().x;
            direction.y = cam.left().z;
            movetype = 3;
        }
        if keys.just_released(KeyCode::KeyD) {
            timers.right.reset();
        }
        if timers.right.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyD) {
            direction.x = cam.right().x;
            direction.y = cam.right().z;
            movetype = 4;
        }

        if direction != Vec2::ZERO && has_dashed == false {
            // Add dash status effect
            let status_dash = StatusEffectDash {
                dash_duration: Timer::new(Duration::from_secs_f32(2.0), TimerMode::Once),
            };
            commands.entity(player).insert(status_dash);

            // Sending events
            movement_event_writer.send(MovementAction::Dash(direction.normalize_or_zero()));
            animation_type_writer.send(AnimationType::MoveType(movetype));
        }
    }
}

pub fn keyboard_jump(
    keys: Res<ButtonInput<KeyCode>>,
    q_1: Query<Has<Grounded>, With<PlayerGroundCollider>>,
    mut q_2: Query<&mut Limit>,
    mut movement_event_writer: EventWriter<MovementAction>,
    mut animation_type_writer: EventWriter<AnimationType>,
) {
    let movetype: u8 = 5;
    for is_grounded in q_1.iter() {
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
