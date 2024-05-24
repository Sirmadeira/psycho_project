use super::link_animations::AnimationEntityLink;
use crate::mod_char_plugin::assemble_parts::find_child_with_name_containing::find_child_with_name_containing;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// Store the correlations
#[derive(Resource, Debug)]
pub struct StoreStartTailCollider(Vec<StartTailCollider>);

// Bunch of entities needed to calculate the midp
#[derive(Component, Debug)]
struct StartTailCollider {
    start: Entity,
    tail: Entity,
    collider: Entity,
}

// Helper function
fn create_collider(
    translation: Vec3,
    collider: Collider
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
    ];

    let special_bones = [
    find_child_with_name_containing(&all_entities_with_children, &names, &main_entity, "Torso").expect("Unique torso bone to exist"),   
    find_child_with_name_containing(&all_entities_with_children, &names, &main_entity, "Foot.R").expect("Unique lower feet leg to exist"),
    find_child_with_name_containing(&all_entities_with_children, &names, &main_entity, "Foot.L").expect("Unique lower feet leg to exist"),
    find_child_with_name_containing(&all_entities_with_children, &names, &main_entity, "Neck").expect("Unique lower feet leg to exist"),];


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
        let half_height = trans1.distance(trans2)/2.0;
        

        let new_collider = create_collider(mid_point,Collider::cylinder(half_height, 0.06));

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


    // Hard coded bones
    for bone in special_bones{
        let name = names.get(bone).unwrap();
        let trans1 = global_transforms
            .get(bone)
            .unwrap()
            .translation();
        if name.as_str() == "Torso"{
            commands.spawn(create_collider(trans1, Collider::cylinder(0.2, 0.15)));
        }
        if name.as_str() == "Foot.L"{
            commands.spawn(create_collider(trans1, Collider::cuboid(0.05, 0.05, 0.15)));
        }
        if name.as_str() == "Foot.R"{
            commands.spawn(create_collider(trans1, Collider::cuboid(0.05, 0.05, 0.15)));
        }
        if name.as_str() == "Neck"{
            commands.spawn(create_collider(trans1, Collider::cuboid(0.15, 0.15, 0.1)));
        }
    }


    commands.insert_resource(StoreStartTailCollider(store_start_tail_collider));
}

// Do ensure this runs after the animations commands in postupdate
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

            // Angvel target doesnt need the end point in this case, since the ro
            let q_difference = start_t.1 * current_t.1.inverse();

            let (axis, angle) = q_difference.to_axis_angle();

            let angvel = Vec3::new(
                axis[0] * angle / dt,
                axis[1] * angle / dt,
                axis[2] * angle / dt,
            );
        
            current_vel.linvel = new_linvel;
            // Clamp to avoid crazy movements
            
            current_vel.angvel = angvel;
        }
    }
}
