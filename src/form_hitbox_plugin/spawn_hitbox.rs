
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::mod_char_plugin::lib::{Skeleton,AmountPlayers};

use crate::mod_char_plugin::helpers::find_child_with_name_containing;
use crate::form_hitbox_plugin::helpers::create_dynamic_collider_groups;
use crate::form_hitbox_plugin::lib::{BaseEntities,PidInfo,Offset,Hitbox};


pub fn spawn_simple_colliders(
    mut commands: Commands,
    all_entities_with_children: Query<&Children>,
    main_entity_option: Query<Entity, With<Skeleton>>,
    names: Query<&Name>,
    global_transforms: Query<&GlobalTransform>,
    player_amount: Res<AmountPlayers>,
) {
    // Main bone entity to search in
    for (main_entity,collision_number) in main_entity_option.iter().zip(1u32..) {

        // Creates dynamic specific groups according to the amount of players
        let collision_groups = create_dynamic_collider_groups(&player_amount, collision_number);

        // Name of bones
        let bone_names = vec![
            "Spine",
            "RightFoot",
            "LeftFoot",
            "Head",
            "RightHand",
            "LeftHand",
        ];
        
        let mut special_bones = Vec::new();

        for bone_name in bone_names {
            let bone = find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &main_entity,
                bone_name,
            ).expect(&format!("Unique {} bone to exist", bone_name));
            
            special_bones.push(bone);
        }
        
        // Hard coded colliders
        for bone in special_bones {
            // Use unwrap_or_else to handle potential None values safely if needed
            let name = names.get(bone).expect("Bone name not found");

            let trans1 = global_transforms
                .get(bone)
                .expect("Global transform not found")
                .translation();

            let (collider, offset) = match name.as_str() {
                name if name.contains("Spine") => (
                    Collider::cylinder(0.2, 0.15),
                    Vec3::ZERO,
                ),
                name if name.contains("RightFoot") => (
                    Collider::cuboid(0.05, 0.10, 0.05),
                    Vec3::ZERO,
                ),
                name if name.contains("LeftFoot") => (
                    Collider::cuboid(0.05, 0.10, 0.05),
                    Vec3::ZERO,
                ),
                name if name.contains("Head") => (
                    Collider::cuboid(0.15, 0.15, 0.1),
                    Vec3::new(0.0, 0.10, 0.00),
                ),
                name if name.contains("RightHand") => (
                    Collider::cuboid(0.05, 0.10, 0.10),
                    Vec3::new(0.00, -0.1, 0.0),
                ),
                name if name.contains("LeftHand") => (
                    Collider::cuboid(0.05, 0.10, 0.10),
                    Vec3::new(0.00, -0.1, 0.0),
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
                        .insert(collision_groups);
                });
        }
    }
}


// WARNING ONLY ADD TO UNIQUE BONES
pub fn spawn_complex_colliders(
    mut commands: Commands,
    all_entities_with_children: Query<&Children>,
    main_entity_option: Query<Entity, With<Skeleton>>,
    names: Query<&Name>,
    global_transforms: Query<&GlobalTransform>,
    player_amount: Res<AmountPlayers>,
) {
    // Main bone entity to search in
    for (main_entity,collision_number) in main_entity_option.iter().zip(1u32..) {
        let collision_groups = create_dynamic_collider_groups(&player_amount, collision_number);
        // Define the bone names we want to find
        let bone_names = vec![
            "LeftArm", "LeftForeArm", "RightArm", "RightForeArm",
            "LeftUpLeg", "LeftLeg", "RightUpLeg", "RightLeg",
            "LeftLeg", "LeftFoot", "RightLeg", "RightFoot",
            "LeftForeArm", "LeftHand", "RightForeArm", "RightHand"
        ];

        let mut bone_entities = Vec::new();

        for bone_name in bone_names {
            let bone = find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &main_entity,
                bone_name,
            ).expect(&format!("Unique {} bone to exist", bone_name));
            
            bone_entities.push(bone);
        }
        
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

            let first_name = names.get(bone_entities[i]).expect("Bone to have name");
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

            let (collider, offset) = match collider_name.as_str() {
                "POT" => (
                    Collider::cylinder(half_height, 0.15),
                    Vec3::ZERO,
                ),
                _ => (
                    Collider::cylinder(half_height, 0.09),
                    Vec3::ZERO,
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
                        .insert(collision_groups)
                        .insert(Sensor);
                });

            // Move to the next pair of elements
            i += 2;
        }
    }
}
