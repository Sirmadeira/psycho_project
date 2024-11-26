use super::MarkerMainCamera;
use crate::shared::protocol::player_structs::*;
use crate::shared::shared_physics::*;
use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::client::input::leafwing::InputSystemSet;
use lightyear::client::prediction::rollback::Rollback;
use lightyear::client::prediction::Predicted;
use lightyear::inputs::leafwing::input_buffer::InputBuffer;
use lightyear::shared::tick_manager::TickManager;

use lightyear::client::prediction::plugin::is_in_rollback;

pub struct PlayerPhysicsPlugin;

impl Plugin for PlayerPhysicsPlugin {
    fn build(&self, app: &mut App) {
        // Add physical components to predicted players
        app.add_systems(Update, add_physics_to_players);

        // Ensures we update the ActionState before buffering them
        app.add_systems(
            FixedPreUpdate,
            camera_rotate_to
                .before(InputSystemSet::BufferClientInputs)
                .run_if(not(is_in_rollback)),
        );
        // It is essential that input bases systems occur in fixedupdate
        app.add_systems(
            FixedUpdate,
            handle_character_actions.in_set(InputPhysicsSet::Input),
        );
    }
}

/// Will add physics to predicted entities
fn add_physics_to_players(
    players: Query<Entity, (With<MarkerPlayer>, Added<Predicted>)>,
    mut commands: Commands,
) {
    for player in players.iter() {
        info!("Adding physics to player");
        commands.entity(player).insert(PlayerPhysics::default());
    }
}

// fn draw_cursor(
//     camera_query: Query<(Entity, &Camera), With<MarkerMainCamera>>,
//     ground_query: Query<Entity, With<FloorMarker>>,
//     global_transforms: Query<&GlobalTransform>,
//     windows: Query<&Window>,
//     mut gizmos: Gizmos,
// ) {
//     if let Ok(ground_entity) = ground_query.get_single() {
//         if let Ok(ground_global_transform) = global_transforms.get(ground_entity) {
//             let (cam_entity, camera) = camera_query.single();
//             if let Ok(camera_global_transform) = global_transforms.get(cam_entity) {
//                 let Some(cursor_position) = windows.single().cursor_position() else {
//                     return;
//                 };
//                 // Calculate a ray pointing from the camera into the world based on the cursor's position.
//                 let Some(ray) = camera.viewport_to_world(camera_global_transform, cursor_position)
//                 else {
//                     return;
//                 };

//                 // Calculate if and where the ray is hitting the ground plane.
//                 let Some(distance) = ray.intersect_plane(
//                     ground_global_transform.translation(),
//                     InfinitePlane3d::new(ground_global_transform.up()),
//                 ) else {
//                     return;
//                 };
//                 let point = ray.get_point(distance);

//                 // Draw a circle just above the ground plane at that position.
//                 gizmos.circle(
//                     point + ground_global_transform.up() * 0.5,
//                     ground_global_transform.up(),
//                     1.0,
//                     Color::WHITE,
//                 );
//             }
//         }
//     }
// }

fn camera_rotate_to(
    q_transform: Query<&Transform, With<MarkerMainCamera>>,
    mut player_action_state: Query<&mut ActionState<PlayerAction>, With<Predicted>>,
) {
    if let Ok(cam_transform) = q_transform.get_single() {
        let (yaw, pitch, _) = cam_transform.rotation.to_euler(EulerRot::YXZ);
        if let Ok(mut action_state) = player_action_state.get_single_mut() {
            action_state.set_axis_pair(&PlayerAction::RotateToCamera, Vec2::new(pitch, yaw));
        }
    }
}

/// Process character actions and apply them to their associated character
/// entity.
fn handle_character_actions(
    time: Res<Time>,
    mut query: Query<
        (
            &ActionState<PlayerAction>,
            &InputBuffer<PlayerAction>,
            &RayHits,
            CharacterQuery,
        ),
        With<Predicted>,
    >,
    tick_manager: Res<TickManager>,
    rollback: Option<Res<Rollback>>,
) {
    // Get the current tick. It may be apart of a rollback.
    let tick = rollback
        .as_ref()
        .map(|rb| tick_manager.tick_or_rollback_tick(rb))
        .unwrap_or(tick_manager.tick());

    for (action_state, input_buffer, ray_hits, mut character) in &mut query {
        // Use the current character action if it is.
        if input_buffer.get(tick).is_some() {
            apply_character_action(&time, action_state, ray_hits, &mut character);
            continue;
        }

        // If the current character action is not real then use the last real
        // character action.
        if let Some((_, prev_action_state)) = input_buffer.get_last_with_tick() {
            apply_character_action(&time, prev_action_state, ray_hits, &mut character);
        } else {
            // No inputs are in the buffer yet. This can happen during initial
            // connection. Apply the default input (i.e. nothing pressed).
            apply_character_action(&time, action_state, ray_hits, &mut character);
        }
    }
}
