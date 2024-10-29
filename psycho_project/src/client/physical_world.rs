use crate::shared::protocol::world_structs::FloorMarker;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use lightyear::shared::replication::components::Replicated;

pub struct PhysicalWorldPlugin;

impl Plugin for PhysicalWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_world);
    }
}

fn spawn_world(
    floor: Query<Entity, (Added<Replicated>, With<FloorMarker>)>,
    mut commands: Commands,
) {
    if let Ok(floor) = floor.get_single() {
        info!("Spawning physical floor");
        // Usually it is recommended that this is a shared bundle but for now fuck it
        let collider = Collider::cuboid(100.0, 0.5, 100.0);
        let name = Name::new("PhysicalFloor");
        commands.entity(floor).insert(collider).insert(name);
    }
}
