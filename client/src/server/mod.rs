use crate::shared::protocol::lobby_structs::Lobbies;
use bevy::prelude::*;

mod lobby_systems;

mod server_systems;

use self::{lobby_systems::*, server_systems::*};

pub struct ExampleServerPlugin;

impl Plugin for ExampleServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Lobbies::default());
        app.insert_resource(PlayerAmount::default());
        app.register_type::<Lobbies>();
        app.add_systems(Startup, (init, start_server));

        // What happens when you connects to server
        app.add_systems(Update, handle_connections);

        app.add_systems(Update, create_lobby);
    }
}
