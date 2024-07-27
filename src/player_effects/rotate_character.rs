
use bevy::prelude::*;
use bevy::animation::AnimationTarget;
use std::f32::consts::PI;
use crate::spawn_game_entities::lib::*;
use crate::spawn_game_entities::helpers::find_child_with_name_containing;

// Guy who is gonna send animation nevents according to rotation also is gonna tell to rotate the dynamic player
pub fn detect_rotation(q_1: Query<&Transform,With<CamInfo>>,q_2:Query<&Transform,With<Player>>){
    
    let camera_transform = q_1.get_single().expect("Cam to have transform");

    let player_transform = q_2.get_single().expect("Player to have transform");

    let camera_forward = camera_transform.forward();

    let player_forward = player_transform.forward();
    
    // Calculates difference in yaw angle between two vectorsee
    let dot_product = camera_forward.dot(*player_forward);

    let angle = dot_product.acos();

    // Define a threshold for yaw difference
    let threshold = PI / 4.0; // 45 degrees

    // Check if the angle exceeds the threshold
    if angle > threshold {
        println!("Angle difference{}",angle);
        println!("Angle difference{}",threshold);
    }


    // todo!()
}


pub fn spine_look_at(
    q_1: Query<&Transform, With<CamInfo>>,
    q_2: Query<Entity, With<Player>>,
    children_entities: Query<&Children>,
    names: Query<&Name>,
    mut transform: Query<&mut Transform, Without<CamInfo>>,
    mut commands: Commands,
) {
    let target_transform = q_1.get_single().expect("Failed to find camera transform");
    let player = q_2.get_single().expect("Failed to find player entity");

    let spine = find_child_with_name_containing(&children_entities, &names, &player, "Spine_2")
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
    let yaw_limits = (PI /3.0, PI);

    let clipped_yaw = if yaw > 0.0 {
        yaw.clamp(yaw_limits.0, yaw_limits.1)
    } else {
        yaw.clamp(-yaw_limits.1, -yaw_limits.0)
    };

    // Convert the clipped yaw and pitch back to a direction vector
    let clipped_direction = Vec3::new(
        clipped_pitch.cos() * clipped_yaw.sin(),
        clipped_pitch.sin(),
        clipped_pitch.cos() * clipped_yaw.cos(),
    );

    // Set the up vector (typically this is the world's up vector, e.g., Vec3::Y)
    let up = Vec3::Y;

    *current_transform = current_transform.looking_at(clipped_direction, up);
}

// Refactor this make it so it send an event that rotates the root bone and triggers when a certain yaw is achieved for example
// If camera rotates 90 degress from starting position, spin character
// pub fn player_look_at_camera(
//     q_1: Query<&Transform, With<CamInfo>>,
//     q_2: Query<(&Transform, &PdInfo), With<Player>>,
//     mut q_3: Query<&mut Velocity, With<Player>>,
// ) {
//     let cam_transform = q_1.get_single().expect("Camera to exist");
//     let (player_transform, pd_info) = q_2.get_single().expect("Player to exist");

//     let rot_error = (cam_transform.rotation * player_transform.rotation.inverse()).normalize();

//     let (axis_error, angle_error) = rot_error.to_axis_angle();

//     let angle_error_rad = angle_error.to_radians();

//     let angvel = pd_info.kp * angle_error_rad * axis_error;

//     for mut v in q_3.iter_mut() {
//         v.angvel = angvel;
//     }
// }



// // Interpolates root bone after animation
// pub fn apply_diagonal(query: Query<Entity,With<AnimatedEntity>>,
//     dig_infos: Query<Option<&DiagonalAnimation>,Added<DiagonalAnimation>>,
//     names: Query<&Name>,
//     children_entities:Query<&Children>,
//     mut transform: Query<&mut Transform>,
//     mut commands: Commands){

//     let entity = query.get_single().expect("TO have animated entity");

//     for dig_info in dig_infos.iter(){


//         if let Some(dig_info) = dig_info{

//             let root = find_child_with_name_containing(&children_entities, &names, &entity, "Root").expect("To have root");

//             commands.entity(root).remove::<AnimationTarget>();

//             let mut current_transform = transform.get_mut(root).expect("To have transform as ALL  entities in bevy");
        
        
//             let target_transform = match dig_info.0.as_str() {
//                 "RightDigWalk" => Quat::from_rotation_z(-45.0_f32.to_radians()),
//                 "LeftDigWalk" => Quat::from_rotation_z(45.0_f32.to_radians()),
//                 "BackRightDigWalk" => Quat::from_rotation_z(45.0_f32.to_radians()),
//                 "BackLeftDigWalk" => Quat::from_rotation_z(-45.0_f32.to_radians()),
//                 _ => Quat::IDENTITY, // Default rotation if animation name doesn't match any case
//             };
        
//             current_transform.rotation = current_transform.rotation.slerp(target_transform, 0.25);   
//         } 
    
//     }
// }