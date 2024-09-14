use crate::shared::protocol::lobby_structs::Lobbies;
use bevy::prelude::*;

mod server_systems;

use self::server_systems::*;

pub struct ExampleServerPlugin;

impl Plugin for ExampleServerPlugin {
    fn build(&self, app: &mut App) {
        // Initializing resources
        app.insert_resource(Lobbies::default());
        app.insert_resource(PlayerAmount::default());
        // Debug registering
        app.register_type::<Lobbies>();

        // Initializing sever current has head
        app.add_systems(Startup, (init, start_server));

        // What happens when you connects to server
        app.add_systems(Update, handle_connections);
        // What happens when you disconnect from server
        app.add_systems(Update, handle_disconnections);
        // Creates a lobby
        app.add_systems(Update, create_lobby);
    }
}
