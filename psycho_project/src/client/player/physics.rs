use crate::shared::physics::CharacterPhysicsBundle;
use crate::shared::protocol::player_structs::*;
use bevy::prelude::*;
use lightyear::client::prediction::Predicted;

pub struct PlayerPhysicsPlugin;

impl Plugin for PlayerPhysicsPlugin {
    fn build(&self, app: &mut App) {
        // Add physical components to predicted players
        app.add_systems(Update, add_physics_to_players);
    }
}

fn add_physics_to_players(
    players: Query<Entity, (Added<Predicted>, With<PlayerId>)>,
    mut commands: Commands,
) {
    for player in players.iter() {
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
