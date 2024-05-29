use super::link_animations::AnimationEntityLink;
use crate::mod_char_plugin::assemble_parts::find_child_with_name_containing::find_child_with_name_containing;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// Store the correlations
#[derive(Resource, Debug)]
pub struct StoreStartTailCollider(Vec<StartTailCollider>);

// Bunch of entities needed to calculate the mid transform
#[derive(Component, Debug)]
struct StartTailCollider {
    start: Entity,
    tail: Entity,
    collider: Entity,
}

// Colliders are not based on another collider axis
#[derive(Component, Debug)]
pub struct BoneCollider(Entity);

// Stores the offset of the specific collider
#[derive(Reflect, Component, Debug)]
pub struct Offset(Vec3);

#[derive(Reflect,Component,Debug)]
pub struct PidInfo{
    // Proportional gain how agressive to reac
    kp: f32,
    // Integral gain accumulated error over time
    ki: f32,
    // Derivative gain predicts future error
    kd:f32,
    // These values are here because they need to be agregated
    integral: Vec3,
    previous_error: Vec3
}


// Helper function
fn create_collider(
    translation: Vec3,
    collider: Collider,
) -> (
    RigidBody,
    SpatialBundle,
    GravityScale,
    Collider,
    Velocity,
    CollisionGroups,
) {
    (
        RigidBody::Dynamic,
        SpatialBundle {
            transform: Transform::from_translation(translation),
            ..Default::default()
        },
        GravityScale(0.0),
        collider,
        Velocity::zero(),
        CollisionGroups::new(Group::GROUP_2, Group::NONE),
    )
}

// WARNING ONLY ADD TO UNIQUE BONES
pub fn spawn_colliders(
    mut commands: Commands,
    all_entities_with_children: Query<&Children>,
    main_entity_option: Query<Entity, With<AnimationEntityLink>>,
    names: Query<&Name>,
    global_transforms: Query<&GlobalTransform>,
) {
    // Main bone entity to search in
    let Ok(main_entity) = main_entity_option.get_single() else {
        println!("No player entity available");
        return;
    };

    // Bone entities to be collected adjust as needed
    let bone_entities = [
        // find_child_with_name_containing(&all_entities_with_children, &names, &main_entity, "Torso").expect("Unique torso bone to exist"),
        find_child_with_name_containing(
            &all_entities_with_children,
            &names,
            &main_entity,
            "UpperArm.R",
        )
        .expect("Unique upper right arm to exist"),
        find_child_with_name_containing(
            &all_entities_with_children,
            &names,
            &main_entity,
            "LowerArm.R",
        )
        .expect("Unique lower right arm to exist"),
        find_child_with_name_containing(
            &all_entities_with_children,
            &names,
            &main_entity,
            "UpperArm.L",
        )
        .expect("Unique upper left arm to exist"),
        find_child_with_name_containing(
            &all_entities_with_children,
            &names,
            &main_entity,
            "LowerArm.L",
        )
        .expect("Unique lower left arm to exist"),
        find_child_with_name_containing(
            &all_entities_with_children,
            &names,
            &main_entity,
            "UpperLeg.R",
        )
        .expect("Unique upper right leg to exist"),
        find_child_with_name_containing(
            &all_entities_with_children,
            &names,
            &main_entity,
            "LowerLeg.R",
        )
        .expect("Unique lower right leg to exist"),
        find_child_with_name_containing(
            &all_entities_with_children,
            &names,
            &main_entity,
            "UpperLeg.L",
        )
        .expect("Unique upper left leg to exist"),
        find_child_with_name_containing(
            &all_entities_with_children,
            &names,
            &main_entity,
            "LowerLeg.L",
        )
        .expect("Unique lower left leg to exist"),
        find_child_with_name_containing(
            &all_entities_with_children,
            &names,
            &main_entity,
            "LowerLeg.L",
        )
        .expect("Unique lower left leg to exist"),
        find_child_with_name_containing(
            &all_entities_with_children,
            &names,
            &main_entity,
            "Foot.L",
        )
        .expect("Unique lower left leg to exist"),
        find_child_with_name_containing(
            &all_entities_with_children,
            &names,
            &main_entity,
            "LowerLeg.R",
        )
        .expect("Unique lower left leg to exist"),
        find_child_with_name_containing(
            &all_entities_with_children,
            &names,
            &main_entity,
            "Foot.R",
        )
        .expect("Unique lower left leg to exist"),
        find_child_with_name_containing(
            &all_entities_with_children,
            &names,
            &main_entity,
            "LowerArm.R",
        )
        .expect("Unique lower right arm to exist"),
        find_child_with_name_containing(
            &all_entities_with_children,
            &names,
            &main_entity,
            "Wrist.R",
        )
        .expect("Unique wrist to exist"),
        find_child_with_name_containing(
            &all_entities_with_children,
            &names,
            &main_entity,
            "LowerArm.L",
        )
        .expect("Unique lower right arm to exist"),
        find_child_with_name_containing(
            &all_entities_with_children,
            &names,
            &main_entity,
            "Wrist.L",
        )
        .expect("Unique wrist to exist"),
    ];

    let special_bones = [
        find_child_with_name_containing(&all_entities_with_children, &names, &main_entity, "Torso")
            .expect("Unique torso bone to exist"),
        find_child_with_name_containing(
            &all_entities_with_children,
            &names,
            &main_entity,
            "Foot.R",
        )
        .expect("Unique lower feet to exist"),
        find_child_with_name_containing(
            &all_entities_with_children,
            &names,
            &main_entity,
            "Foot.L",
        )
        .expect("Unique lower feet to exist"),
        find_child_with_name_containing(&all_entities_with_children, &names, &main_entity, "Neck")
            .expect("Unique lower feet leg to exist"),
        find_child_with_name_containing(
            &all_entities_with_children,
            &names,
            &main_entity,
            "Index1.L",
        )
        .expect("Unique Index1 to exist"),
        find_child_with_name_containing(
            &all_entities_with_children,
            &names,
            &main_entity,
            "Index1.R",
        )
        .expect("Unique Index1 to exist"),
    ];

    // Use this when you want to create a collider between bones
    let mut store_start_tail_collider = vec![];
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

        let mid_point = Vec3::new(
            (trans1.x + trans2.x) / 2.0,
            (trans1.y + trans2.y) / 2.0,
            (trans1.z + trans2.z) / 2.0,
        );
        let half_height = trans1.distance(trans2) / 2.0;

        let new_collider = create_collider(mid_point, Collider::cylinder(half_height, 0.06));

        let new_collider_id = commands.spawn(new_collider).id();

        let start_and_tail = StartTailCollider {
            start: bone_entities[i],
            tail: bone_entities[i + 1],
            collider: new_collider_id,
        };

        store_start_tail_collider.push(start_and_tail);

        // Move to the next pair of elements
        i += 2;
    }

    // Hard coded colliders
    for bone in special_bones {
        // Use unwrap_or_else to handle potential None values safely if needed
        let name = names.get(bone).expect("Bone name not found");

        let trans1 = global_transforms
            .get(bone)
            .expect("Global transform not found")
            .translation();

        let col_name = Name::new(format!("{}_col", name));

        let (collider, offset) = match name.as_str() {
            "Torso" => (Collider::cylinder(0.2, 0.15), Vec3::ZERO),
            "Foot.L" => (Collider::cuboid(0.05, 0.05, 0.05), Vec3::ZERO),
            "Foot.R" => (Collider::cuboid(0.05, 0.05, 0.05), Vec3::ZERO),
            "Neck" => (Collider::cuboid(0.15, 0.15, 0.1), Vec3::new(0.0, 0.10, 0.00)),
            "Index1.R" => (
                Collider::cuboid(0.05, 0.10, 0.10),
                Vec3::new(0.00, -0.1, 0.0),
            ),
            "Index1.L" => (
                Collider::cuboid(0.05, 0.10, 0.10),
                Vec3::new(0.00, -0.1, 0.0),
            ),
            _ => continue, // Skip bones that are not in the list
        };

        commands
            .spawn(create_collider(trans1 + offset, collider))
            .insert(BoneCollider(bone))
            .insert(Offset(offset))
            .insert(col_name)
            .insert(PidInfo{
                kp: 50.0,
                ki: 15.0,
                kd: 0.1,
                integral: Vec3::ZERO,
                previous_error: Vec3::ZERO,
            });
    }

    commands.insert_resource(StoreStartTailCollider(store_start_tail_collider));
}

// Do ensure this runs after trasform propagations
pub fn col_follow_animation(
    time: Res<Time>,
    entities: Res<StoreStartTailCollider>,
    transforms: Query<&GlobalTransform>,
    mut velocities: Query<&mut Velocity>,
) {
    for store in entities.0.iter() {
        let dt = time.delta_seconds();

        let start = store.start;
        let tail = store.tail;
        let collider = store.collider;
        // Need to use global transform with linvel because skeletons transforms are parent based and the animation

        if let Ok(mut current_vel) = velocities.get_mut(collider) {
            // Current collider location
            let current_t = transforms
                .get(collider)
                .unwrap()
                .to_scale_rotation_translation();
            // Start bone transform
            let start_t = transforms
                .get(start)
                .unwrap()
                .to_scale_rotation_translation();
            // End bone transform
            let end_t = transforms
                .get(tail)
                .unwrap()
                .to_scale_rotation_translation();

            // Linvel target transform
            let target_t = Vec3::new(
                (start_t.2.x + end_t.2.x) / 2.0,
                (start_t.2.y + end_t.2.y) / 2.0,
                (start_t.2.z + end_t.2.z) / 2.0,
            );
            // Sorvete
            let new_linvel = (target_t - current_t.2) / dt;

            current_vel.linvel = new_linvel;
            // Angvel target doesnt need the end point in this case, since the ro
            let q_difference = start_t.1 * current_t.1.inverse();

            let (axis, angle) = q_difference.to_axis_angle();

            let angvel = Vec3::new(
                axis[0] * angle / dt,
                axis[1] * angle / dt,
                axis[2] * angle / dt,
            );

            // Not ideal but what you gonna do - wait for bevy 0.14 to fix
            if angle > 5.55 || angle < 0.01 {
                current_vel.angvel = Vec3::splat(0.0);
            } else {
                current_vel.angvel = angvel;
            }
        }
    }
}

//
pub fn hard_colliders_look_at(
    mut collider_info: Query<
        (&mut Velocity, &Transform, &BoneCollider,&mut PidInfo, Option<&Offset>),
        With<BoneCollider>,
    >,
    bone_transform: Query<&GlobalTransform>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();

    for (mut vel, current_transform, &BoneCollider(target_entity),mut pid, offset) in
        collider_info.iter_mut()
    {
        // Just grabbing target bone transform
        let target_transform = bone_transform
            .get(target_entity)
            .expect("Bone to have transform")
            .compute_transform();


        // Doin this so it considers this offset
        let target_translation = if let Some(offset) = offset {
            target_transform.translation + offset.0
        } else {
            target_transform.translation
        };

        // Distance vector
        let error = target_translation - current_transform.translation; 

        // Calculating integral and derivative
        pid.integral += error * dt;

        // Derivative to avoid future errors
        let derivative = (error - pid.previous_error)/dt;

        // Aggregate last error for later interaction
        pid.previous_error = error;

        let output = pid.kp * error + pid.ki * pid.integral  + derivative * pid.kd;

        vel.linvel = output;

        // Angvel ios fucked until 14
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
