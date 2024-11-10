use crate::shared::protocol::lobby_structs::*;
use bevy::prelude::*;
use shared_physics::SharedPhysicsPlugin;

pub mod protocol;
pub mod shared_physics;

use self::protocol::ProtocolPlugin;

/// In this plugin you should add all systems/plugins that need to exist both in server and in client
/// Worth noting that most input logic should be here, as you move something in client you should also move in server. When doing client side prediction
#[derive(Clone)]
pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        // Imported plugins
        // Self made plugins
        app.add_plugins(ProtocolPlugin);
        app.add_plugins(SharedPhysicsPlugin);
    }
}
