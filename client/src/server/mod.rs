use crate::shared::protocol::lobby_structs::Lobbies;
use crate::shared::protocol::player_structs::{PlayerBundleMap, PlayerLoadout, PlayerVisuals};
use bevy::prelude::*;
use lightyear::server::events::*;
mod server_systems;

use self::server_systems::*;

pub struct ExampleServerPlugin;

impl Plugin for ExampleServerPlugin {
    fn build(&self, app: &mut App) {
        // Initializing resources
        app.init_resource::<Lobbies>();
        app.init_resource::<PlayerBundleMap>();
        app.init_resource::<PlayerAmount>();

        // Debug registering
        app.register_type::<Lobbies>();
        app.register_type::<PlayerVisuals>();

        // Initializing sever current has head
        app.add_systems(Startup, (init, start_server));

        // What happens when you connects to server
        app.add_systems(Update, handle_connections);

        // What happens when you disconnect from server
        app.add_systems(Update, handle_disconnections);

        // Creates a lobby
        app.add_systems(Update, create_lobby);

        // Listeners
        app.add_systems(Update, listener_player_loadout);
    }
}

// Responsible for changing the loadout right now it just
fn listener_player_loadout(
    mut events: EventReader<MessageEvent<PlayerLoadout>>,
    mut player_map: ResMut<PlayerBundleMap>,
) {
    for event in events.read() {
        let message = event.message();
        let client_id = event.context();

        info!("Receiveing new player loadout from {}", client_id);

        if let Some(player_bundle) = player_map.0.get_mut(client_id) {
            player_bundle.visuals = message.0.clone();
            info!("Found it is bundle and changing it for what client said");
        } else {
            error!("Something went worng in grabing this id info in server");
        }
    }
}
