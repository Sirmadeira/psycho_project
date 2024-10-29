//! Responsible for mantaining all the physical world of our game meaning most collider shall be spawmed and replicated to server
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use lightyear::prelude::server::Replicate;

use crate::shared::{protocol::world_structs::FloorMarker, shared_behavior::FloorPhysicsBundle};

/// Responsible for spawning the entities that are correlated to physics mechanic
pub struct PhysicsWorldPlugin;

impl Plugin for PhysicsWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_floor_collider);
    }
}

/// Spawn in both server and client a single cubicle collider
fn spawn_floor_collider(mut commands: Commands) {
    info!("Spawning server floor and replicating to client");
    commands
        .spawn(FloorPhysicsBundle::default())
        .insert(Replicate::default())
        .insert(Name::new("PhysicalFloor"))
        .insert(FloorMarker);
}
