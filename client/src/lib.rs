use bevy::prelude::*;
use config::build_client;
use lightyear::{client::input::native::InputSystemSet, prelude::MainSet};
use shared::SharedPlugin;

mod config;
mod movement_systems;
mod setup_systems;

use self::{movement_systems::*, setup_systems::*};

pub fn create_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(bevy::log::LogPlugin::default()));
    app.add_plugins(build_client());
    app.add_plugins(SharedPlugin);
    app.add_systems(Startup, (init, start_client));
    app.add_systems(
        PreUpdate,
        (handle_connection, handle_disconnection).after(MainSet::Receive),
    );
    app.add_systems(
        FixedPreUpdate,
        buffer_input.in_set(InputSystemSet::BufferInputs),
    );

    app.add_systems(FixedUpdate, player_movement);

    return app;
}
