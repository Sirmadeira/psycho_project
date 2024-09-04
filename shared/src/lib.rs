use bevy::prelude::*;
use protocol::ProtocolPlugin;

pub mod config;
pub mod protocol;
pub mod sockets;

// Basically a whole structure of common setups
pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProtocolPlugin);
    }
}
