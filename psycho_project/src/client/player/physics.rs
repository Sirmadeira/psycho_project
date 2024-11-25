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
use lightyear::shared::replication::components::Controlled;
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
            capture_mouse_input
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
        commands.entity(player).insert(PhysicsBundle::player());
    }
}

fn capture_mouse_input(
    q_window: Query<&Window>,
    mut q_action_state: Query<
        (&Position, &mut ActionState<PlayerAction>),
        (With<Predicted>, With<Controlled>),
    >,
    q_camera: Query<(&Camera, &Transform, &GlobalTransform), With<MarkerMainCamera>>,
) {
    if let Ok((camera, camera_transform, camera_global_transform)) = q_camera.get_single() {
        if let Ok((player_pos, mut action_state)) = q_action_state.get_single_mut() {
            let window = q_window.single();
            // This will cast a ray from camera
            if let Some(world_position) = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_global_transform, cursor))
                .map(|ray| ray.origin.truncate())
            {
                let mouse_position_relative = world_position - player_pos.0.truncate();
                let camera_scale = camera_transform.scale.x;
                let pair = mouse_position_relative * camera_scale;
                action_state.set_axis_pair(&PlayerAction::MousePositionRelative, pair);
            }
        }
    }
}

/// Process character actions and apply them to their associated character
/// entity.
fn handle_character_actions(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    mut query: Query<
        (
            &ActionState<PlayerAction>,
            &InputBuffer<PlayerAction>,
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

    for (action_state, input_buffer, mut character) in &mut query {
        // Use the current character action if it is.
        if input_buffer.get(tick).is_some() {
            apply_character_action(&time, &spatial_query, action_state, &mut character);
            continue;
        }

        // If the current character action is not real then use the last real
        // character action.
        if let Some((_, prev_action_state)) = input_buffer.get_last_with_tick() {
            apply_character_action(&time, &spatial_query, prev_action_state, &mut character);
        } else {
            // No inputs are in the buffer yet. This can happen during initial
            // connection. Apply the default input (i.e. nothing pressed).
            apply_character_action(&time, &spatial_query, action_state, &mut character);
        }
    }
}
