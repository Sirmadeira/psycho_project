use crate::shared::physics::*;
use crate::shared::protocol::player_structs::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::InputMap;
use leafwing_input_manager::prelude::KeyboardVirtualDPad;
use lightyear::client::prediction::Predicted;
use lightyear::shared::replication::components::Controlled;

pub struct PlayerPhysicsPlugin;

impl Plugin for PlayerPhysicsPlugin {
    fn build(&self, app: &mut App) {
        // Add physical components to predicted players
        app.add_systems(Update, add_physics_to_players);
    }
}

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
        } else {
            info!("Remote character replicated to us: {player:?}");
        }
        commands
            .entity(player)
            .insert(CharacterPhysicsBundle::default());
    }
}

// fn add_physics_to_interpolated(
//     players: Query<Entity, (Added<Interpolated>, With<PlayerId>)>,
//     mut commands: Commands,
// ) {
//     for player in players.iter() {
//         commands
//             .entity(player)
//             .insert(CharacterPhysicsBundle::default());
//     }
// }
