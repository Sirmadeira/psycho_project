use crate::mod_char_plugin::helpers::find_child_with_name_containing;
use crate::player_effects_plugin::lib::{Player, SidePlayer, StatePlayerCreation};

use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy_rapier3d::prelude::*;

pub struct FormHitboxPlugin;

impl Plugin for FormHitboxPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Hitbox>();
        app.register_type::<BaseEntities>();
        app.register_type::<PidInfo>();
        app.register_type::<Offset>();
        app.add_systems(
            OnEnter(StatePlayerCreation::Done),
            (spawn_simple_colliders, spawn_complex_colliders).chain(),
        );
        app.add_systems(
            FixedPostUpdate,
            colliders_look_at
                .run_if(in_state(StatePlayerCreation::Done))
                .after(TransformSystem::TransformPropagate),
        );
    }
}

// Marker component good to check if any of the colliders are touching the ground collider
#[derive(Reflect, Component, Debug)]
pub struct Hitbox;

// Colliders are not based on another collider axis
#[derive(Reflect, Component, Debug)]
pub struct BaseEntities {
    start: Entity,
    end: Option<Entity>,
}

// Stores the offset of the specific collider
#[derive(Reflect, Component, Debug)]
pub struct Offset(Vec3);

#[derive(Reflect, Component, Debug)]
pub struct PidInfo {
    // Proportional gain how agressive to reac
    kp: f32,
    // Integral gain accumulated error over time
    ki: f32,
    // Derivative gain predicts future error
    kd: f32,
    // These values are here because they need to be agregated
    integral: Vec3,
    previous_error: Vec3,
}

pub fn spawn_simple_colliders(
    mut commands: Commands,
    all_entities_with_children: Query<&Children>,
    main_entity_option: Query<Entity, Or<(With<Player>,With<SidePlayer>)>>,
    names: Query<&Name>,
    global_transforms: Query<&GlobalTransform>,
) {
    // Main bone entity to search in
    for main_entity in main_entity_option.iter() {
        // Bones without tail
        let special_bones = [
            find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &main_entity,
                "Spine",
            )
            .expect("Unique spine bone to exist"),
            find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &main_entity,
                "RightFoot",
            )
            .expect("Unique lower feet to exist"),
            find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &main_entity,
                "LeftFoot",
            )
            .expect("Unique lower feet to exist"),
            find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &main_entity,
                "Head",
            )
            .expect("Unique head to exist"),
            find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &main_entity,
                "RightHand",
            )
            .expect("Unique hand to exist"),
            find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &main_entity,
                "LeftHand",
            )
            .expect("Unique hand to exist"),
        ];
        // Hard coded colliders
        for bone in special_bones {
            // Use unwrap_or_else to handle potential None values safely if needed
            let name = names.get(bone).expect("Bone name not found");

            let trans1 = global_transforms
                .get(bone)
                .expect("Global transform not found")
                .translation();

            let (collider, offset, collision_group) = match name.as_str() {
                name if name.contains("Spine") => (
                    Collider::cylinder(0.2, 0.15),
                    Vec3::ZERO,
                    CollisionGroups::new(Group::GROUP_2, Group::NONE),
                ),
                name if name.contains("RightFoot") => (
                    Collider::cuboid(0.05, 0.10, 0.05),
                    Vec3::ZERO,
                    CollisionGroups::new(Group::GROUP_2, Group::NONE),
                ),
                name if name.contains("LeftFoot") => (
                    Collider::cuboid(0.05, 0.10, 0.05),
                    Vec3::ZERO,
                    CollisionGroups::new(Group::GROUP_2, Group::NONE),
                ),
                name if name.contains("Head") => (
                    Collider::cuboid(0.15, 0.15, 0.1),
                    Vec3::new(0.0, 0.10, 0.00),
                    CollisionGroups::new(Group::GROUP_2, Group::NONE),
                ),
                name if name.contains("RightHand") => (
                    Collider::cuboid(0.05, 0.10, 0.10),
                    Vec3::new(0.00, -0.1, 0.0),
                    CollisionGroups::new(Group::GROUP_2, Group::NONE),
                ),
                name if name.contains("LeftHand") => (
                    Collider::cuboid(0.05, 0.10, 0.10),
                    Vec3::new(0.00, -0.1, 0.0),
                    CollisionGroups::new(Group::GROUP_2, Group::NONE),
                ),
                _ => continue, // Skip bones that are not in the list
            };

            commands
                .spawn(RigidBody::Dynamic)
                .insert(Hitbox)
                .insert(BaseEntities {
                    start: bone,
                    end: None,
                })
                .insert(PidInfo {
                    kp: 50.0,
                    ki: 15.0,
                    kd: 0.1,
                    integral: Vec3::ZERO,
                    previous_error: Vec3::ZERO,
                })
                .insert(Offset(offset))
                .insert(SpatialBundle {
                    transform: Transform::from_translation(trans1),
                    ..Default::default()
                })
                .insert(Velocity::zero())
                .with_children(|children| {
                    children
                        .spawn(collider)
                        .insert(Sensor)
                        .insert(collision_group);
                });
        }
    }
}

// WARNING ONLY ADD TO UNIQUE BONES
pub fn spawn_complex_colliders(
    mut commands: Commands,
    all_entities_with_children: Query<&Children>,
    main_entity_option: Query<Entity, Or<(With<Player>,With<SidePlayer>)>>,
    names: Query<&Name>,
    global_transforms: Query<&GlobalTransform>,
) {
    // Main bone entity to search in
    for main_entity in main_entity_option.iter() {
        // Start and end entities for something
        let bone_entities = [
            find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &main_entity,
                "LeftArm",
            )
            .unwrap(),
            find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &main_entity,
                "LeftForeArm",
            )
            .unwrap(),
            find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &main_entity,
                "RightArm",
            )
            .unwrap(),
            find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &main_entity,
                "RightForeArm",
            )
            .unwrap(),
            find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &main_entity,
                "LeftUpLeg",
            )
            .unwrap(),
            find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &main_entity,
                "LeftLeg",
            )
            .unwrap(),
            find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &main_entity,
                "RightUpLeg",
            )
            .unwrap(),
            find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &main_entity,
                "RightLeg",
            )
            .unwrap(),
            find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &main_entity,
                "LeftLeg",
            )
            .unwrap(),
            find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &main_entity,
                "LeftFoot",
            )
            .unwrap(),
            find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &main_entity,
                "RightLeg",
            )
            .unwrap(),
            find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &main_entity,
                "RightFoot",
            )
            .unwrap(),
            find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &main_entity,
                "LeftForeArm",
            )
            .unwrap(),
            find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &main_entity,
                "LeftHand",
            )
            .unwrap(),
            find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &main_entity,
                "RightForeArm",
            )
            .unwrap(),
            find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &main_entity,
                "RightHand",
            )
            .unwrap(),
        ];
        // Create colliders and spawn them
        let mut i = 0;
        while i < bone_entities.len() - 1 {
            let trans1 = global_transforms
                .get(bone_entities[i])
                .unwrap()
                .translation();
            let trans2 = global_transforms
                .get(bone_entities[i + 1])
                .unwrap()
                .translation();

            let first_name = names.get(bone_entities[i]).expect("Bone name not found");
            let last_name = names.get(bone_entities[i + 1]).expect("Bone to have name");

            let collider_name = Name::new(format!(
                "{}_{}_col",
                first_name.to_lowercase(),
                last_name.to_lowercase()
            ));

            // Starting point
            let mid_point = Vec3::new(
                (trans1.x + trans2.x) / 2.0,
                (trans1.y + trans2.y) / 2.0,
                (trans1.z + trans2.z) / 2.0,
            );

            // Distance between the two
            let half_height = trans1.distance(trans2) / 2.0;

            let (collider, offset, collision_group) = match collider_name.as_str() {
                "POT" => (
                    Collider::cylinder(half_height, 0.15),
                    Vec3::ZERO,
                    CollisionGroups::new(Group::GROUP_2, Group::NONE),
                ),
                _ => (
                    Collider::cylinder(half_height, 0.09),
                    Vec3::ZERO,
                    CollisionGroups::new(Group::GROUP_2, Group::NONE),
                ),
            };

            // Optional entity
            let end = if i + 1 < bone_entities.len() {
                Some(bone_entities[i + 1])
            } else {
                None
            };

            commands
                .spawn(RigidBody::Dynamic)
                .insert(Hitbox)
                .insert(BaseEntities {
                    start: bone_entities[i],
                    end: end,
                })
                .insert(PidInfo {
                    kp: 50.0,
                    ki: 15.0,
                    kd: 0.1,
                    integral: Vec3::ZERO,
                    previous_error: Vec3::ZERO,
                })
                .insert(Offset(offset))
                .insert(SpatialBundle {
                    transform: Transform::from_translation(mid_point),
                    ..Default::default()
                })
                .insert(Velocity::zero())
                .with_children(|children| {
                    children
                        .spawn(collider)
                        .insert(collision_group)
                        .insert(Sensor);
                });

            // Move to the next pair of elements
            i += 2;
        }
    }
}

//
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

        target_translation = target_translation + offset.0;
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

        // Angvel is fucked until 14 animation update
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
