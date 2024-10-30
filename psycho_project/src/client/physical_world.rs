use crate::shared::{
    protocol::world_structs::FloorMarker,
    physics::{FloorPhysicsBundle, FLOOR_HEIGHT, FLOOR_WIDTH},
};
use bevy::prelude::*;
use lightyear::shared::replication::components::Replicated;

pub struct PhysicalWorldPlugin;

impl Plugin for PhysicalWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_cosmetic_floor);
    }
}

fn add_cosmetic_floor(
    floor: Query<Entity, (Added<Replicated>, With<FloorMarker>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok(floor) = floor.get_single() {
        info!("Spawning physical floor and adding it is cosmetic");
        // Usually it is recommended that this is a shared bundle but for now fuck it
        let name = Name::new("PhysicalFloor");
        commands
            .entity(floor)
            .insert(FloorPhysicsBundle::default())
            .insert(name)
            .insert(PbrBundle {
                mesh: meshes.add(Cuboid::new(FLOOR_WIDTH, FLOOR_HEIGHT, FLOOR_WIDTH)),
                material: materials.add(Color::srgb(1.0, 1.0, 1.0)),
                ..default()
            });
    }
}
