//! This module contains the shared code between the client and the server.
//!
//! The rendering code is here because you might want to run the example in host-server mode, where the server also acts as a client.
//! The simulation logic (movement, etc.) should be shared between client and server to guarantee that there won't be
//! mispredictions/rollbacks.
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy_mod_picking::DefaultPickingPlugins;

pub mod protocol;
use self::protocol::ProtocolPlugin;

#[derive(Clone)]
pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        // the protocol needs to be shared between the client and server
        app.add_plugins(ProtocolPlugin);
        if app.is_plugin_added::<RenderPlugin>() {
            app.add_plugins(DefaultPickingPlugins);
            app.add_systems(Startup, init);
        }
    }
}

// Common initialization both  in server and client
fn init(mut commands: Commands) {
    let common_camera = Camera2dBundle {
        transform: Transform::from_xyz(2.3, 1.1, 0.0),
        ..Default::default()
    };
    commands.spawn(common_camera);
}
