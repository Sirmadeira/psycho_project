use crate::shared::protocol::lobby_structs::Lobbies;
use bevy::prelude::*;
use lightyear::prelude::server::*;
use lightyear::prelude::*;

mod lobby_systems;

mod core_systems;
mod player_systems;

use self::{core_systems::*, lobby_systems::*, player_systems::*};

pub struct ExampleServerPlugin;

impl Plugin for ExampleServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Lobbies::default());
        app.register_type::<Lobbies>();
        app.add_systems(Startup, (init, start_server));
        // the physics/FixedUpdates systems that consume inputs should be run in this set
        app.add_systems(FixedUpdate, movement);
        app.add_systems(Update, send_message);

        // What happens when yopu connects
        app.add_systems(Update, (handle_connections).run_if(is_host_server));
        // What happens when any player discontects
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
