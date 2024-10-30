use avian3d::prelude::*;
use bevy::prelude::*;
use common::shared::FIXED_TIMESTEP_HZ;
use lightyear::prelude::*;
use shared_behavior::SharedBehaviorPlugin;

pub mod protocol;
pub mod shared_behavior;

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
        app.add_plugins(InputPlugin::<Inputs>::default());
        app.add_plugins(
            PhysicsPlugins::new(FixedUpdate)
                .build()
                .disable::<ColliderHierarchyPlugin>(),
        )
        .insert_resource(Time::new_with(Physics::fixed_once_hz(FIXED_TIMESTEP_HZ)))
        .insert_resource(Gravity(Vec3::ZERO));
        // Shared debuging
        app.register_type::<PlayerVisuals>();
        app.register_type::<PlayerBundleMap>();
        app.register_type::<Lobbies>();
        // Self made plugins
        app.add_plugins(ProtocolPlugin);
        app.add_plugins(SharedBehaviorPlugin);
    }
}
