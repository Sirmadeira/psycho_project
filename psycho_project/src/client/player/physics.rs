use crate::shared::protocol::player_structs::*;
use crate::shared::shared_physics::*;
use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::client::prediction::rollback::Rollback;
use lightyear::client::prediction::Predicted;
use lightyear::inputs::leafwing::input_buffer::InputBuffer;
use lightyear::shared::replication::components::Controlled;
use lightyear::shared::tick_manager::TickManager;
pub struct PlayerPhysicsPlugin;

impl Plugin for PlayerPhysicsPlugin {
    fn build(&self, app: &mut App) {
        // Add physical components to predicted players
        app.add_systems(Update, add_physics_to_players);

        // It is essential that input bases systems occur in fixedupdate
        app.add_systems(
            FixedUpdate,
            handle_character_actions.in_set(InputPhysicsSet::Input),
        );
    }
}

/// Will add physics to predicted entities
fn add_physics_to_players(
    players: Query<(Entity, Has<Controlled>), (Added<Predicted>, With<PlayerId>)>,
    mut commands: Commands,
) {
    for (player, is_controlled) in players.iter() {
        if is_controlled {
            info!("Adding InputMap to controlled and predicted entity {player:?}");
            commands.entity(player).insert(
                InputMap::new([(CharacterAction::Jump, KeyCode::Space)])
                    .with_dual_axis(CharacterAction::Move, KeyboardVirtualDPad::WASD),
            );
        }
        // Inserted position here to avoid inside spawning
        commands
            .entity(player)
            .insert(PhysicsBundle::player());
    }
}

/// Process character actions and apply them to their associated character
/// entity.
fn handle_character_actions(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    mut query: Query<
        (
            &ActionState<CharacterAction>,
            &InputBuffer<CharacterAction>,
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
