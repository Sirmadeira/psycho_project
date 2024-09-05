use bevy::prelude::*;

use config::build_client;

mod config;
mod setup_systems;

use crate::network_client::setup_systems::*;

pub struct ConfigurableClient;

impl Plugin for ConfigurableClient {
    fn build(&self, app: &mut App) {
        app.add_plugins(build_client());
        app.add_systems(Startup, (init, start_client));
    }
}
