use super::link_animations::AnimationEntityLink;
use crate::mod_char_plugin::assemble_parts::find_child_with_name_containing;
use bevy::prelude::*;
use bevy_rapier3d:: prelude::*;


// Colliders are not based on another collider axis
#[derive(Component, Debug)]
pub struct BaseEntities{
    start: Entity,
    end: Option<Entity>
}

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
pub fn spawn_complex_colliders(
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

    // Start and end entities for something
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
        let last_name = names.get(bone_entities[i+1]).expect("Bone to have name");

        let collider_name = Name::new(format!("{}_{}_col", first_name.to_lowercase(),last_name.to_lowercase()));

        // Starting point
        let mid_point = Vec3::new(
            (trans1.x + trans2.x) / 2.0,
            (trans1.y + trans2.y) / 2.0,
            (trans1.z + trans2.z) / 2.0,
        );

        // Distance between the two
        let half_height = trans1.distance(trans2) / 2.0;

        let (collider,offset) = match collider_name.as_str() {
            "POT" => (Collider::cylinder(half_height, 0.15), Vec3::ZERO),
            _ =>(Collider::cylinder(half_height, 0.09), Vec3::ZERO)
        };

        // Optional entity
        let end = if i + 1 < bone_entities.len() {
            Some(bone_entities[i + 1])
        } else {
            None
        };


        commands.spawn(create_collider(mid_point,collider))
        .insert(collider_name)
        .insert(Offset(offset))
        .insert(BaseEntities{start: bone_entities[i],end: end})
        .insert(PidInfo{
            kp: 50.0,
            ki: 15.0,
            kd: 0.1,
            integral: Vec3::ZERO,
            previous_error: Vec3::ZERO,
        });
        // Move to the next pair of elements
        i += 2;
    }
}

pub fn spawn_simple_colliders(  mut commands: Commands,
    all_entities_with_children: Query<&Children>,
    main_entity_option: Query<Entity, With<AnimationEntityLink>>,
    names: Query<&Name>,
    global_transforms: Query<&GlobalTransform>,){

    // Main bone entity to search in
    let Ok(main_entity) = main_entity_option.get_single() else {
        println!("No player entity available");
        return;
    };
    // Bones without tail
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
     // Hard coded colliders
     for bone in special_bones {
        // Use unwrap_or_else to handle potential None values safely if needed
        let name = names.get(bone).expect("Bone name not found");

        let trans1 = global_transforms
            .get(bone)
            .expect("Global transform not found")
            .translation();

        let col_name = Name::new(format!("{}_col", name.to_lowercase()));

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
            .spawn(create_collider(trans1, collider))
            .insert(BaseEntities{start:bone,end: None})
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
}



//
pub fn simple_colliders_look_at(
    mut collider_info: Query<
        (&mut Velocity, &Transform, &BaseEntities,&mut PidInfo, &Offset),
        With<BaseEntities>,
    >,
    bone_transform: Query<&GlobalTransform>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();

    for (mut vel, current_transform,base_entities,mut pid, offset) in
        collider_info.iter_mut()
    {   

        // Start bone for the simple ones
        let target_transform =  bone_transform.get(base_entities.start).expect("Start bone global transform").compute_transform();
        let mut target_translation = target_transform.translation;

        // End and start bone position 
        if let Some(end) = base_entities.end{
            let trans1  = bone_transform.get(base_entities.start).expect("Start bone global transform").translation();
            let trans2 = bone_transform.get(end).expect("End bone global transform").translation();
            target_translation =  Vec3::new(
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
