use crate::shared::protocol::lobby_structs::Lobbies;
use bevy::prelude::*;
use lightyear::prelude::server::*;
use lightyear::prelude::*;

mod lobby_systems;

mod server_systems;

use self::{server_systems::*, lobby_systems::*};

pub struct ExampleServerPlugin;

impl Plugin for ExampleServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Lobbies::default());
        app.register_type::<Lobbies>();
        app.add_systems(Startup, (init, start_server));

        // What happens when you connects to server
        app.add_systems(Update, (handle_connections).run_if(is_host_server));
        // What happens when any player disconnects
        app.add_systems(
            Update,
            handle_disconnections.run_if(in_state(NetworkingState::Started)),
        );
        // Systems that update general lobbies resource
        app.add_systems(
            Update,
            (handle_lobby_join, handle_lobby_exit, handle_start_game).run_if(is_mode_separate),
        );
    }
}
