use avian3d::prelude::Position;
use crate::shared::{
    protocol::world_structs::FloorMarker,
    physics::{FloorPhysicsBundle, FLOOR_HEIGHT, FLOOR_WIDTH},
};
use lightyear::client::interpolation::VisualInterpolationPlugin;
use lightyear::client::interpolation::VisualInterpolateStatus;
use bevy::prelude::*;
use lightyear::shared::replication::components::Replicated;
use lightyear::client::components::Confirmed;

/// Anything correlated to general physics should be placed in this pluign
pub struct PhysicalWorldPlugin;

impl Plugin for PhysicalWorldPlugin {
    fn build(&self, app: &mut App) {
        // Set up visual interp plugins for Position and Rotation. This doesn't
        // do anything until you add VisualInterpolationStatus components to
        // entities.
        app.add_plugins(VisualInterpolationPlugin::<Position>::default());
        app.observe(add_visual_interpolation_components::<Position>);

        app.add_systems(Update, add_cosmetic_physics_floor);
    }
}

/// This guy will add visual interpolation component to anyone that is not confirmed. or predicted
/// Basically also made to avoid stuttering
fn add_visual_interpolation_components<T: Component>(
    trigger: Trigger<OnAdd, T>,
    query: Query<Entity, (With<T>, Without<Confirmed>, Without<FloorMarker>)>,
    mut commands: Commands,
) {
    if !query.contains(trigger.entity()) {
        return;
    }
    debug!("Adding visual interp component to {:?}", trigger.entity());
    // We must trigger change detection so that the SyncPlugin will
    // detect and sync changes from Position/Rotation to Transform.
    //
    // Without syncing interpolated pos/rot to transform, things like
    // sprites, meshes, and text which render based on the *Transform*
    // component (not avian's Position) will be stuttery.
    commands
        .entity(trigger.entity())
        .insert(VisualInterpolateStatus::<T> {
            trigger_change_detection: true,
            ..default()
        });
}

/// Adds cosmeticc to the replicated floor entity given by server also adds it physics
fn add_cosmetic_physics_floor(
    floor: Query<Entity, (Added<Replicated>, With<FloorMarker>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok(floor) = floor.get_single() {
        info!("Spawning physical floor and adding it is cosmetic");
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
