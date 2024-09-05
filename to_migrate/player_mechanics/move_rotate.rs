use crate::form_ingame_camera::setup_entities::CamInfo;
use crate::form_modular_char::helpers::find_child_with_name_containing;
use crate::form_player::lib::*;
use crate::player_mechanics::MovementAction;
use bevy::animation::AnimationTarget;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

pub fn move_character(
    mut movement_event_reader: EventReader<MovementAction>,
    time: Res<Time>,
    mut q_1: Query<(&mut Velocity, &mut ExternalImpulse, &PlayerVel), With<Player>>,
) {
    for event in movement_event_reader.read() {
        for (mut vel, mut impulse, player_vel) in &mut q_1 {
            match event {
                MovementAction::Move(direction) => {
                    vel.linvel.x += direction.x * player_vel.linvel * time.delta_seconds();
                    vel.linvel.z += direction.y * player_vel.linvel * time.delta_seconds();
                }
                MovementAction::Dash(direction) => {
                    impulse.impulse.x = direction.x * player_vel.dash_vel;
                    impulse.impulse.z = direction.y * player_vel.dash_vel;
                }
                MovementAction::Jump => vel.linvel.y += player_vel.jump_vel,
            }
        }
    }
}

// Guy who is gonna send animation events according to rotation also is gonna tell to rotate the dynamic player
pub fn rotate_character(
    q_1: Query<&Transform, With<CamInfo>>,
    q_2: Query<(&Transform, &PlayerVel), With<Player>>,
    mut q_3: Query<&mut Velocity, With<Player>>,
) {
    let camera_transform = q_1.get_single().expect("Cam to have transform");

    let (player_transform, player_vel) = q_2.get_single().expect("Player to have transform");

    let rot_error = (camera_transform.rotation * player_transform.rotation.inverse()).normalize();

    let (axis_error, angle_error) = rot_error.to_axis_angle();

    let angle_error_rad = angle_error.to_radians();

    let angvel = player_vel.ang_vel * angle_error_rad * axis_error;

    let only_y = Vec3::new(0.0, angvel.y, 0.0);

    if only_y.y > 0.18 || only_y.y < -0.18 {
        for mut vel in q_3.iter_mut() {
            vel.angvel = only_y;
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

    let spine = find_child_with_name_containing(&children_entities, &names, &player, "Head")
        .expect("Failed to find spine bone");

    // Remove animation target
    commands.entity(spine).remove::<AnimationTarget>();

    let mut current_transform = transform
        .get_mut(spine)
        .expect("Failed to get spine transform");

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
        0.0,
        clipped_pitch.sin(),
        clipped_pitch.cos() * clipped_yaw.cos(),
    );

    // Set the up vector (typically this is the world's up vector, e.g., Vec3::Y)
    let up = Vec3::Y;

    *current_transform = current_transform.looking_at(clipped_direction, up);
}
