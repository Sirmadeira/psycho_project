use bevy::prelude::*;

mod lobby_systems;

mod player_systems;

use self::{lobby_systems::*, player_systems::*};

pub struct ExampleServerPlugin;

impl Plugin for ExampleServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (init, start_server));
        // the physics/FixedUpdates systems that consume inputs should be run in this set
        app.add_systems(FixedUpdate, movement);
        app.add_systems(Update, (send_message, handle_connections));
    }
}
