//! Responsible for mantaining all the physical world of our game meaning most collider shall be spawmed and replicated to server
use crate::shared::protocol::world_structs::FloorMarker;
use crate::shared::shared_physics::PhysicsBundle;
use avian3d::prelude:: Position;
use bevy::prelude::*;
use lightyear::prelude::server::Replicate;

/// Responsible for spawning the entities that are correlated to physics mechanic
pub struct PhysicsWorldPlugin;

impl Plugin for PhysicsWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_floor_collider);

        // app.add_systems(FixedUpdate, shared_gravity_force);
    }
}

/// Spawn in both server and client a single cubicle collider
fn spawn_floor_collider(mut commands: Commands) {
    info!("Spawning server floor and replicating to client");
    commands
        .spawn(PhysicsBundle::floor())
        .insert(FloorMarker)
        .insert(Replicate::default())
        .insert(Name::new("PhysicalFloor"))
        .insert(Position(Vec3::new(0.0, -1.5, 0.0)));
}
