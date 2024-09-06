use bevy::prelude::*;

use shared::SharedPlugin;

mod config;

mod movement_systems;
mod setup_systems;

use crate::config::build_server_plugin;

use self::{movement_systems::*, setup_systems::*};

pub fn create_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(bevy::log::LogPlugin::default()));
    app.add_plugins(build_server_plugin());
    app.add_plugins(SharedPlugin);
    app.add_systems(Startup, (init, start_server));
    app.add_systems(Update, (handle_connections, handle_disconnections));
    app.add_systems(FixedUpdate, movement);

    return app;
}
