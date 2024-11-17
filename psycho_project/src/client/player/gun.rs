use crate::shared::shared_gun::shared_spawn_bullet;
use crate::shared::shared_physics::InputPhysicsSet;
use bevy::prelude::*;
use lightyear::client::prediction::plugin::is_in_rollback;

/// Responsible for gun related mechanics
pub struct PlayerGunPlugin;

impl Plugin for PlayerGunPlugin {
    fn build(&self, app: &mut App) {
        // Done so it avoid double bullet spawn else server spawns it
        app.add_systems(
            FixedUpdate,
            shared_spawn_bullet
                .run_if(not(is_in_rollback))
                .in_set(InputPhysicsSet::Input),
        );
    }
}
