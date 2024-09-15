use crate::shared::protocol::lobby_structs::Lobbies;
use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

mod server_systems;

use self::server_systems::*;

pub struct ExampleServerPlugin;

impl Plugin for ExampleServerPlugin {
    fn build(&self, app: &mut App) {
        // Initializing resources
        app.init_resource::<Lobbies>();
        app.init_resource::<PlayerAmount>();
        // Debug registering
        app.register_type::<Lobbies>();
        app.add_plugins(ResourceInspectorPlugin::<Lobbies>::default());

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
