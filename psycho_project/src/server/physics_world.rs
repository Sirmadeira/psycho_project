use crate::shared::protocol::player_structs::CommonChannel;
use crate::shared::protocol::world_structs::*;
use crate::shared::shared_physics::PhysicsBundle;
use avian3d::prelude::Position;
use bevy::prelude::*;
use lightyear::prelude::server::Replicate;
use lightyear::prelude::*;
use lightyear::shared::replication::network_target::NetworkTarget;
use server::SyncTarget;
/// Responsible for spawning the entities that are correlated to physics mechanic
pub struct PhysicsWorldPlugin;

impl Plugin for PhysicsWorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CycleTimer>();
        app.add_systems(Startup, replicate_resource);
        app.add_systems(Startup, spawn_floor_collider);
        app.add_systems(Startup, spawn_server_sun);
        app.add_systems(FixedUpdate, tick_sun_cycle);
    }
}

fn replicate_resource(mut commands: Commands) {
    commands.replicate_resource::<CycleTimer, CommonChannel>(NetworkTarget::All);
}

/// Spawn in both server and client a single cubicle collider
fn spawn_floor_collider(mut commands: Commands) {
    info!("Spawning server floor and replicating to client");
    commands
        .spawn(PhysicsBundle::floor())
        .insert(FloorMarker)
        .insert(Replicate::default())
        .insert(Name::new("PhysicalFloor"))
        .insert(Position(Vec3::new(0.0, -1.25, 0.0)));
}

/// Spawns server sun
fn spawn_server_sun(mut commands: Commands) {
    commands
        .spawn(SunMarker)
        .insert(Transform::default())
        .insert(Replicate {
            target: ReplicationTarget {
                target: NetworkTarget::All,
                ..default()
            },
            sync: SyncTarget {
                interpolation: NetworkTarget::All,
                ..default()
            },
            ..default()
        });
}

fn tick_sun_cycle(mut cycle_time: ResMut<CycleTimer>, time: Res<Time>) {
    cycle_time.0.tick(time.delta());
}
