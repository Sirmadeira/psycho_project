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

// Do ensure this runs after the animations commands
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
                // SORVETE FDP
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
