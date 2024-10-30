use bevy::prelude::*;
use physics::SharedPhysicsPlugin;

pub mod physics;
pub mod protocol;

use self::protocol::ProtocolPlugin;
use crate::shared::protocol::lobby_structs::*;
use crate::shared::protocol::player_structs::*;

/// In this plugin you should add all systems/plugins that need to exist both in server and in client
/// Worth noting that most input logic should be here, as you move something in client you should also move in server. When doing client side prediction
#[derive(Clone)]
pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        // Imported plugins
        // Shared debuging
        app.register_type::<PlayerVisuals>();
        app.register_type::<PlayerBundleMap>();
        app.register_type::<Lobbies>();
        // Self made plugins
        app.add_plugins(ProtocolPlugin);
        app.add_plugins(SharedPhysicsPlugin);
    }
}
