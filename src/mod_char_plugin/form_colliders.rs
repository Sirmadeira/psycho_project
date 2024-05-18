use super::link_animations::AnimationEntityLink;
use crate::mod_char_plugin::assemble_parts::find_child_with_name_containing::find_child_with_name_containing;

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier3d::prelude::*;



#[derive(Resource)]
pub struct ColAndBone(pub HashMap<Entity, Entity>);


// WARNING ONLY ADD TO UNIQUE BONES
// FILL THE RESOURCE
pub fn spawn_colliders(
    mut commands: Commands,
    all_entities_with_children: Query<&Children>,
    main_entity_option: Query<Entity, With<AnimationEntityLink>>,
    names: Query<&Name>,
    global_transforms: Query<&GlobalTransform>
) {
    let Ok(main_entity) = main_entity_option.get_single() else {
        println!("No player entity available");
        return;
    };


    let mut col_entities_by_name = HashMap::new();
    // Getting unique bone entity
    let torso_entity_option =
        find_child_with_name_containing(&all_entities_with_children, &names, &main_entity, "Torso");

    // If the bone exists grab it is global transform and insert into the resourceee
    if let Some(torso_entity) = torso_entity_option {
        // Creating rigibody with a collider
        println!("Found torso bone for collider {:?}",torso_entity);

        let torso_translation = global_transforms.get(torso_entity).unwrap().translation();

        let torso_col = (
            RigidBody::Dynamic,
            SpatialBundle {
                transform:Transform::from_translation(torso_translation),
                ..Default::default()
            },
            GravityScale(0.0),
            Collider::ball(0.25),
            Velocity::zero(),
            CollisionGroups::new(Group::GROUP_2, Group::NONE),
        );
        let torso_col_entity = commands.spawn(torso_col).id();
        col_entities_by_name.insert(torso_col_entity,torso_entity);

    }

    commands.insert_resource(ColAndBone(col_entities_by_name));

}


pub fn col_follow_animation(
    entities: ResMut<ColAndBone>,
    transforms: Query<&Transform>,
    mut velocities: Query<&mut Velocity>,
) {
    // The interpolation will be almost immediate
    let mut current_time = 0.0; // Current time, starting from 0
    let total_s = 0.5; // Max s value of interpolation in seconds
    let dt = 1.0 / 60.0; // Time step for interpolation, adjust as needed
    
    for (current,target) in entities.0.iter(){

        let mut current_vel_c = velocities.get_mut(*current);
        let current_t = transforms.get(*current).unwrap().rotation;
        let target_t = transforms.get(*target).unwrap().rotation;

        // println!("current{}",current_t);
        // println!("target {}",target_t);
        

        for v in current_vel_c.iter_mut(){
            while current_time < total_s {
                
                let s = current_time / total_s;

                let interpolated_q = current_t.slerp(target_t, s);

                let q_difference = interpolated_q * current_t.inverse();

                let (axis, angle) = q_difference.to_axis_angle();

                let angvel = (
                    axis[0] * angle / dt,
                    axis[1] * angle / dt,
                    axis[2] * angle / dt,
                );

                v.angvel = angvel.into();

                current_time += dt;
            }
        }

    }
}