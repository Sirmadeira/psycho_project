use bevy::prelude::*;
use clap::Parser;
use config::build_client;
use shared::SharedPlugin;

mod config;
mod setup_systems;
use self::setup_systems::*;

#[derive(Parser, PartialEq, Debug)]
pub struct Cli;

pub fn create_app(cli: Cli) -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(bevy::log::LogPlugin::default()));
    app.add_plugins(build_client());
    app.add_plugins(SharedPlugin);
    app.add_systems(Startup, (init, start_client));
    return app;
}
