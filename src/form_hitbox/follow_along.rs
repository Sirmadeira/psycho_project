use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::form_hitbox::lib::{BaseEntities, Offset, PidInfo};

pub fn colliders_look_at(
    mut collider_info: Query<
        (
            &mut Velocity,
            &Transform,
            &BaseEntities,
            &mut PidInfo,
            &Offset,
        ),
        With<BaseEntities>,
    >,
    bone_transform: Query<&GlobalTransform>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();

    for (mut vel, current_transform, base_entities, mut pid, offset) in collider_info.iter_mut() {
        // Start bone for the simple ones
        let target_transform = bone_transform
            .get(base_entities.start)
            .expect("Start bone global transform")
            .compute_transform();
        let mut target_translation = target_transform.translation;

        // End and start bone position
        if let Some(end) = base_entities.end {
            let trans1 = bone_transform
                .get(base_entities.start)
                .expect("Start bone global transform")
                .translation();
            let trans2 = bone_transform
                .get(end)
                .expect("End bone global transform")
                .translation();
            target_translation = Vec3::new(
                (trans1.x + trans2.x) / 2.0,
                (trans1.y + trans2.y) / 2.0,
                (trans1.z + trans2.z) / 2.0,
            );
        }

        let rotated_offset = target_transform.rotation * offset.0;

        target_translation = target_translation + rotated_offset;
        // Distance vector
        let error = target_translation - current_transform.translation;

        // Calculating integral and derivative
        pid.integral += error * dt;

        // Derivative to avoid future errors
        let derivative = (error - pid.previous_error) / dt;

        // Aggregate last error for later interaction
        pid.previous_error = error;

        let output = pid.kp * error + pid.ki * pid.integral + derivative * pid.kd;

        vel.linvel = output;

        let desired_rotation = target_transform.rotation * current_transform.rotation.inverse();

        let (axis, angle) = desired_rotation.to_axis_angle();

        let angvel = (
            axis[0] * angle / dt,
            axis[1] * angle / dt,
            axis[2] * angle / dt,
        );

        vel.angvel = angvel.into();
    }
}
