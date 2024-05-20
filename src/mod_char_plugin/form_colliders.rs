use super::link_animations::AnimationEntityLink;
use crate::mod_char_plugin::assemble_parts::find_child_with_name_containing::find_child_with_name_containing;

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier3d::prelude::*;



#[derive(Resource)]
pub struct ColAndBone(pub HashMap<Entity, Entity>);

// Helper function
fn create_collider(translation: Vec3) -> (RigidBody, SpatialBundle, GravityScale, Collider, Velocity, CollisionGroups) {
    (
        RigidBody::Dynamic,
        SpatialBundle {
            transform: Transform::from_translation(translation),
            ..Default::default()
        },
        GravityScale(0.0),
        Collider::ball(0.10),
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
    global_transforms: Query<&GlobalTransform>
) {
    // Main bone entity to search in 
    let Ok(main_entity) = main_entity_option.get_single() else {
        println!("No player entity available");
        return;
    };

    // Hashmap for resource
    let mut col_entities_by_name = HashMap::new();

    // Bone entities to be collected adjust as needed
    let bone_entities = [
        find_child_with_name_containing(&all_entities_with_children, &names, &main_entity, "Torso").expect("Unique torso bone to exist"),
        find_child_with_name_containing(&all_entities_with_children, &names, &main_entity, "UpperArm.R").expect("Unique upper right arm to exist"),
        find_child_with_name_containing(&all_entities_with_children, &names, &main_entity, "UpperArm.L").expect("Unique upper left arm to exist"),
        find_child_with_name_containing(&all_entities_with_children, &names, &main_entity, "LowerArm.R").expect("Unique lower right arm to exist"),
        find_child_with_name_containing(&all_entities_with_children, &names, &main_entity, "LowerArm.L").expect("Unique lower left arm to exist"),
        find_child_with_name_containing(&all_entities_with_children, &names, &main_entity, "UpperLeg.R").expect("Unique upper right leg to exist"),
        find_child_with_name_containing(&all_entities_with_children, &names, &main_entity, "UpperLeg.L").expect("Unique upper left leg to exist"),
        find_child_with_name_containing(&all_entities_with_children, &names, &main_entity, "LowerLeg.R").expect("Unique lower right leg to exist"),
        find_child_with_name_containing(&all_entities_with_children, &names, &main_entity, "LowerLeg.L").expect("Unique lower left leg to exist"),
        find_child_with_name_containing(&all_entities_with_children, &names, &main_entity, "Foot.R").expect("Unique lower feet leg to exist"),
        find_child_with_name_containing(&all_entities_with_children, &names, &main_entity, "Foot.L").expect("Unique lower feet leg to exist"),
    ];

    // Retrieve global transforms for the bone entities
    let global_transforms_result = global_transforms.get_many(bone_entities);
    let global_transforms = match global_transforms_result {
        Ok(transforms) => transforms,
        Err(_) => {
            println!("Failed to get global transforms for bone entities");
            return;
        }
    };

    // Create colliders and spawn them
    for (trans, &bone_entity) in global_transforms.iter().zip(bone_entities.iter()) {
        let new_collider = create_collider(trans.translation());
        let new_collider_id = commands.spawn(new_collider).id();
        col_entities_by_name.insert(new_collider_id, bone_entity);
    }
    commands.insert_resource(ColAndBone(col_entities_by_name));

}




// Do ensure this runs after the animations commands in postupdate
pub fn col_follow_animation(
    time: Res<Time>,
    entities: ResMut<ColAndBone>,
    transforms: Query<&GlobalTransform>,
    mut velocities: Query<&mut Velocity>,
) {
    
    for (current,target) in entities.0.iter(){
        let dt = time.delta_seconds(); 

            if let Ok (mut current_vel) = velocities.get_mut(*current){
                let current_t = transforms.get(*current).unwrap().to_scale_rotation_translation();
                let target_t = transforms.get(*target).unwrap().to_scale_rotation_translation();
                // Need to use global transform with linvel because skeletons transforms are parent based and the animation
                let new_linvel = (target_t.2-current_t.2)/dt;
                let q_difference = target_t.1 * current_t.1.inverse();


                let (axis, angle) = q_difference.to_axis_angle();

                let angvel = (
                    axis[0] * angle / dt,
                    axis[1] * angle / dt,
                    axis[2] * angle / dt,
                );
                current_vel.linvel = new_linvel;
                current_vel.angvel = angvel.into();
            }
    }
}
