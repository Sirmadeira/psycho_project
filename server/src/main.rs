use bevy::prelude::*;
use shared::SharedPlugin;

mod config;

mod setup_systems;

use crate::config::build_server_plugin;

use crate::setup_systems::*;

fn main() {
    // Responsible to start the server
    println!("I am server");

    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(build_server_plugin());
    app.add_plugins(SharedPlugin);
    app.add_systems(Startup, (init, start_server));
    app.run();
}
