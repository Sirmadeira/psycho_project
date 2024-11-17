use bevy::prelude::*;
use shared_gun::SharedGunPlugin;
use shared_physics::SharedPhysicsPlugin;

pub mod diagnostics;
pub mod protocol;
pub mod shared_gun;
pub mod shared_physics;

use self::diagnostics::CentralDiagnosticsPlugin;
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
        app.add_plugins(SharedGunPlugin);
        app.add_plugins(CentralDiagnosticsPlugin);
    }
}
