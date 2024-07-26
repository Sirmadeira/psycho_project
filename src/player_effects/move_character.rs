use bevy::animation::AnimationTarget;
use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

use crate::spawn_game_entities::helpers::find_child_with_name_containing;
use crate::spawn_game_entities::lib::*;
use crate::player_effects::*;
use crate::treat_animations::lib::AnimationType;

use super::TypeOfAttack;

pub fn keyboard_walk(
    keys: Res<ButtonInput<KeyCode>>,
    mut movement_event_writer: EventWriter<MovementAction>,
    mut animation_type_writer: EventWriter<AnimationType>,
    mut attack_writer: EventWriter<TypeOfAttack>,
    q_1: Query<&Transform, With<CamInfo>>,
) {
    let cam = q_1.get_single().expect("To have camera");

    let mut direction = Vec2::ZERO;

    let mut movetype = AnimationType::None;

    let mut attacktype = TypeOfAttack::None;

    //forward
    if keys.pressed(KeyCode::KeyW) {
        direction.x = cam.forward().x;
        direction.y = cam.forward().z;
        movetype = AnimationType::FrontWalk;
        attacktype = TypeOfAttack::Forward;
    }
    // back
    if keys.pressed(KeyCode::KeyS) {
        direction.x = cam.back().x;
        direction.y = cam.back().z;
        movetype = AnimationType::BackWalk;
        attacktype = TypeOfAttack::Backward;
    }
    // left
    if keys.pressed(KeyCode::KeyA) {
        direction.x = cam.left().x;
        direction.y = cam.left().z;
        movetype = AnimationType::LeftWalk;
        attacktype = TypeOfAttack::Left;
    }
    // right
    if keys.pressed(KeyCode::KeyD) {
        direction.x = cam.right().x;
        direction.y = cam.right().z;
        movetype = AnimationType::RightWalk;
        attacktype = TypeOfAttack::Right;
    }
    // Dig right movement
    if keys.pressed(KeyCode::KeyD) && keys.pressed(KeyCode::KeyW) {
        direction.x = cam.right().x + cam.forward().x / 2.0;
        direction.y = cam.right().z + cam.forward().z / 2.0;
        movetype = AnimationType::RightDigWalk;
        attacktype = TypeOfAttack::Right;
    }
    // Dig back right movement
    if keys.pressed(KeyCode::KeyD) && keys.pressed(KeyCode::KeyS) {
        direction.x = cam.right().x + cam.back().x / 2.0;
        direction.y = cam.right().z + cam.back().z / 2.0;
        movetype = AnimationType::BackRightDigWalk;
        attacktype = TypeOfAttack::Right;
    }
    // Dig left movement
    if keys.pressed(KeyCode::KeyA) && keys.pressed(KeyCode::KeyW) {
        direction.x = cam.left().x + cam.forward().x / 2.0;
        direction.y = cam.left().z + cam.forward().z / 2.0;
        movetype = AnimationType::LeftDigWalk;
        attacktype = TypeOfAttack::Left;
    }
    // Dig back left movement
    if keys.pressed(KeyCode::KeyA) && keys.pressed(KeyCode::KeyS) {
        direction.x = cam.left().x + cam.back().x / 2.0;
        direction.y = cam.left().z + cam.back().x / 2.0;
        movetype = AnimationType::BackLeftDigWalk;
        attacktype = TypeOfAttack::Left;
    }

    if direction != Vec2::ZERO {
        movement_event_writer.send(MovementAction::Move(direction.normalize_or_zero()));
        animation_type_writer.send(movetype);
        attack_writer.send(attacktype);
    }
}

pub fn keyboard_dash(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut movement_event_writer: EventWriter<MovementAction>,
    // mut animation_type_writer: EventWriter<AnimationType>,
    mut q: Query<(&mut Timers, Entity, Has<StatusEffectDash>), With<Player>>,
    q_1: Query<&Transform, With<CamInfo>>,
) {
    for (mut timers, player, has_dashed) in q.iter_mut() {
        let cam = q_1.get_single().expect("To have camera");

        let mut direction = Vec2::ZERO;
        // let mut movetype = AnimationType::None;

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
        }
        if keys.just_released(KeyCode::KeyS) {
            timers.down.reset();
        }
        if timers.down.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyS) {
            direction.x = cam.back().x;
            direction.y = cam.back().z;
        }
        if keys.just_released(KeyCode::KeyA) {
            timers.left.reset();
        }
        if timers.left.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyA) {
            direction.x = cam.left().x;
            direction.y = cam.left().z;
        }
        if keys.just_released(KeyCode::KeyD) {
            timers.right.reset();
        }
        if timers.right.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyD) {
            direction.x = cam.right().x;
            direction.y = cam.right().z;
        }

        if direction != Vec2::ZERO && has_dashed == false {
            // Add dash status effect
            let status_dash = StatusEffectDash {
                dash_cooldown: Timer::new(Duration::from_secs_f32(2.0), TimerMode::Once),
            };
            commands.entity(player).insert(status_dash);

            // Sending events
            movement_event_writer.send(MovementAction::Dash(direction.normalize_or_zero()));
            // animation_type_writer.send(movetype);
        }
    }
}

// pub fn keyboard_attack(
//     mouse: Res<ButtonInput<MouseButton>>,
//     mut animation_type_writer: EventWriter<AnimationType>,
//     mut type_attack: EventReader<TypeOfAttack>,
// ) {
//     // Light attack
//     for event in type_attack.read() {
//         match event {
//             TypeOfAttack::Forward => {
//                 if mouse.just_pressed(MouseButton::Left) {
//                 }
//             }
//             TypeOfAttack::Backward => {
//                 if mouse.just_pressed(MouseButton::Left) {
//                 }
//             }
//             TypeOfAttack::Left => {
//                 if mouse.just_pressed(MouseButton::Left) {
//                 }
//             }
//             TypeOfAttack::Right => {
//                 if mouse.just_pressed(MouseButton::Left) {
//                 }
//             }
//             TypeOfAttack::None => {
//                 // Handle no attack state if necessary
//             }
//         }
//     }
//     // Defend
//     if mouse.just_pressed(MouseButton::Right) {
//     }
// }

pub fn keyboard_jump(
    keys: Res<ButtonInput<KeyCode>>,
    q_1: Query<Has<Grounded>, With<PlayerGroundCollider>>,
    mut q_2: Query<&mut Limit>,
    mut movement_event_writer: EventWriter<MovementAction>,
    // mut animation_type_writer: EventWriter<AnimationType>,
) {
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
                    status.impulse.x = direction.x * 200.0;
                    status.impulse.z = direction.y * 200.0;
                }
                MovementAction::Jump => vel.linvel.y += 5.0,
            }
        }
    }
}

pub fn head_look_at(
    q_1: Query<&Transform, With<CamInfo>>,
    q_2: Query<Entity, With<Player>>,
    children_entities: Query<&Children>,
    names: Query<&Name>,
    mut transform: Query<&mut Transform, Without<CamInfo>>,
    mut commands: Commands,
) {
    let target_transform = q_1.get_single().expect("Failed to find camera transform");
    let player = q_2.get_single().expect("Failed to find player entity");

    let head = find_child_with_name_containing(&children_entities, &names, &player, "Spine_2")
        .expect("Failed to find head bone");

    // Remove animation target
    commands.entity(head).remove::<AnimationTarget>();

    let mut current_transform = transform
        .get_mut(head)
        .expect("Failed to get head transform");

    // Compute the direction to look at, using the camera's forward direction
    let target_direction = target_transform.forward();

    // Create a new direction vector with the reversed y component
    let direction =
        Vec3::new(target_direction.x, -target_direction.y, target_direction.z).normalize();

    // Left and right
    let yaw = direction.x.atan2(direction.z);

    // Up and down
    let pitch = direction.y.asin();

    // Clip the pitch to a certain range, e.g., -45 to 45 degrees
    let pitch_limit = PI / 4.0; // 45 degrees
    let clipped_pitch = pitch.clamp(-pitch_limit, pitch_limit);

    //Yaw need to be clipped according to radian quadrants. Meaning it needs to stay between 2 quadrant and 4 quadrant
    // Just think that first limit is inversed
    let yaw_limits = (PI / 1.25, PI);

    let clipped_yaw = if yaw > 0.0 {
        yaw.clamp(yaw_limits.0, yaw_limits.1)
    } else {
        yaw.clamp(-yaw_limits.1, -yaw_limits.0)
    };

    // Convert the clipped yaw and pitch back to a direction vector
    let clipped_direction = Vec3::new(
        clipped_pitch.cos() * clipped_yaw.sin(),
        clipped_pitch.sin(),
        clipped_pitch.cos() * clipped_yaw.cos(),
    );

    // Set the up vector (typically this is the world's up vector, e.g., Vec3::Y)
    let up = Vec3::Y;

    *current_transform = current_transform.looking_at(clipped_direction, up);
}

// pub fn player_look_at_camera(
//     q_1: Query<&Transform, With<CamInfo>>,
//     q_2: Query<(&Transform, &PdInfo), With<Player>>,
//     mut q_3: Query<&mut Velocity, With<Player>>,
// ) {
//     let cam_transform = q_1.get_single().expect("Camera to exist");
//     let (player_transform, pd_info) = q_2.get_single().expect("Player to exist");

//     let rot_error = (cam_transform.rotation * player_transform.rotation.inverse()).normalize();

//     let (axis_error, angle_error) = rot_error.to_axis_angle();

//     let angle_error_rad = angle_error.to_radians();

//     let angvel = pd_info.kp * angle_error_rad * axis_error;

//     for mut v in q_3.iter_mut() {
//         v.angvel = angvel;
//     }
// }
