//! RESPONSIBILITIES - HANDLES ALL MODULAR CHARACTERS CREATIONS

use crate::shared::protocol::player_structs::*;
use bevy::prelude::*;

use lightyear::client::events::*;

pub struct CreateCharPlugin;

impl Plugin for CreateCharPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, naming_player);
    }
}

/// Example system to handle ComponentInsertEvent events
pub(crate) fn naming_player(
    player_id: Query<&PlayerId>,
    mut reader: EventReader<ComponentInsertEvent<PlayerId>>,
    mut commands: Commands,
) {
    for event in reader.read() {
        info!(
            "Creating character according to insertion of component sent from server: {:?}",
            event.entity()
        );
        // Insert identifier for egui
        match player_id.get(event.entity()) {
            Ok(player_id) => {
                let client_id = player_id.0;
                let new_name = Name::new(format!("Player {:?}", client_id));
                commands.entity(event.entity()).insert(new_name);
            }
            Err(error) => {
                // Handle the error, logging or printing it
                error!("Error grabbing client_id: {:?}", error);
            }
        }
    }
}
