use crate::shared::protocol::world_structs::*;
use crate::shared::protocol::CommonChannel;
use crate::shared::shared_physics::FloorPhysics;
use avian3d::prelude::Position;
use bevy::prelude::*;
use lightyear::prelude::server::Replicate;
use lightyear::prelude::*;
use lightyear::shared::replication::network_target::NetworkTarget;

/// Responsible for spawning the entities that are correlated to physics mechanic
pub struct PhysicsWorldPlugin;

impl Plugin for PhysicsWorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CycleTimer>();
        app.add_systems(Startup, replicate_resource);
        app.add_systems(Startup, spawn_floor_collider);
        //This needs refacotring make it tick based TODO
        app.add_systems(FixedUpdate, tick_sun_cycle);
    }
}

// This need refactoring make it tick based to avoid excess of bandwith usage
fn replicate_resource(mut commands: Commands) {
    commands.replicate_resource::<CycleTimer, CommonChannel>(NetworkTarget::All);
}

/// Spawn in both server and client a single cubicle collider
fn spawn_floor_collider(mut commands: Commands) {
    info!("Spawning server floor and replicating to client");
    commands
        .spawn(FloorPhysics::default())
        .insert(FloorMarker)
        .insert(Replicate::default())
        .insert(Name::new("PhysicalFloor"))
        .insert(Position(Vec3::new(0.0, 0.0, 0.0)));
}

fn tick_sun_cycle(mut cycle_time: ResMut<CycleTimer>, time: Res<Time>) {
    cycle_time.0.tick(time.delta());
}
