use crate::shared::protocol::player_structs::ClientInfoBundle;
use crate::shared::protocol::world_structs::FloorMarker;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use lightyear::client::events::ConnectEvent;

use super::MarkerMainCamera;

/// Plugin made to do server culling, could be usefull when making interactions
pub struct ClientReplicatePlayerPlugin;

impl Plugin for ClientReplicatePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_client_info);
        app.add_systems(Update, see_cursor_projection);

        // app.add_systems(
        //     FixedUpdate,
        //     handle_look_at
        //         .in_set(InputPhysicsSet::Input)
        //         .after(change_look_at),
        // );
    }
}

fn spawn_client_info(mut commands: Commands, mut connection_event: EventReader<ConnectEvent>) {
    for event in connection_event.read() {
        let client_id = event.client_id();
        commands.spawn(ClientInfoBundle::new(client_id, Vec3::ZERO));

        info!("Spawning client side replicated info for {}", client_id);
    }
}

/// Based on https://bevy-cheatbook.github.io/cookbook/cursor2world.html
/// Might be super usefull for interaction
fn see_cursor_projection(
    q_camera: Query<(&Camera, &GlobalTransform), With<MarkerMainCamera>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_plane: Query<&GlobalTransform, With<FloorMarker>>,
) {
    if let Ok(ground_transform) = q_plane.get_single() {
        let (camera, camera_transform) = q_camera.single();

        // There is only one primary window, so we can similarly get it from the query:
        let window = q_window.single();

        let Some(cursor_position) = window.cursor_position() else {
            info!("Your curso is outside of bounds");
            return;
        };

        // Mathematically, we can represent the ground as an infinite flat plane.
        // To do that, we need a point (to position the plane) and a normal vector
        // (the "up" direction, perpendicular to the ground plane).
        let plane_origin = ground_transform.translation();
        let plane = InfinitePlane3d::new(ground_transform.up());

        let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            // if it was impossible to compute for whatever reason; we can't do anything
            return;
        };

        // do a ray-plane intersection test, giving us the distance to the ground
        let Some(distance) = ray.intersect_plane(plane_origin, plane) else {
            // If the ray does not intersect the ground
            // (the camera is not looking towards the ground), we can't do anything
            return;
        };
        // use the distance to compute the actual point on the ground in world-space
        let global_cursor = ray.get_point(distance);

        // eprintln!(
        //     "Global cursor coords: {}/{}/{}",
        //     global_cursor.x, global_cursor.y, global_cursor.z
        // );

        // to compute the local coordinates, we need the inverse of the plane's transform
        let inverse_transform_matrix = ground_transform.compute_matrix().inverse();
        let local_cursor = inverse_transform_matrix.transform_point3(global_cursor);

        // eprintln!("Local cursor coords: {}/{}", local_cursor.x, local_cursor.z);
    }
}
