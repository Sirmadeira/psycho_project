//! This module contains the shared code between the client and the server.
//!
//! The rendering code is here because you might want to run the example in host-server mode, where the server also acts as a client.
//! The simulation logic (movement, etc.) should be shared between client and server to guarantee that there won't be
//! mispredictions/rollbacks.
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use lightyear::prelude::*;

pub mod protocol;
pub mod shared_behavior;

use self::protocol::ProtocolPlugin;
use crate::shared::protocol::player_structs::Inputs;
use crate::shared::shared_behavior::update_transform;

/// In this plugin you should add all systems/plugins that need to exist both in server and in client
#[derive(Clone)]
pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        // Imported plugins
        app.add_plugins(InputPlugin::<Inputs>::default());
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
        app.add_plugins(RapierDebugRenderPlugin::default());

        // Shared input systems
        app.add_systems(Update, update_transform);

        // Self made plugins
        app.add_plugins(ProtocolPlugin);
    }
}
