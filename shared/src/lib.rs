use bevy::prelude::*;
use protocol::ProtocolPlugin;

pub mod config;
pub mod drawn_systems;
pub mod movement_systems;
pub mod protocol;
pub mod sockets;

use self::drawn_systems::*;
// Basically a whole structure of common setups
pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProtocolPlugin);
        app.add_systems(Update, draw_boxes);
    }
}
