use crate::form_ingame_camera::setup_entities::CamInfo;
use crate::form_player::lib::*;
use bevy::prelude::*;

pub fn sync_player_camera(
    player_q: Query<&Transform, With<Player>>,
    mut cam_q: Query<(&mut CamInfo, &mut Transform), Without<Player>>,
) {
    let player = player_q.get_single().expect("Player to exist");
    let (cam, mut cam_transform) = cam_q.get_single_mut().expect("Camera to exist");

    let rotation_matrix = Mat3::from_quat(cam_transform.rotation);

    let desired_translation = rotation_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam.zoom.radius));
    // Update the camera translation
    cam_transform.translation = desired_translation + player.translation;
}
