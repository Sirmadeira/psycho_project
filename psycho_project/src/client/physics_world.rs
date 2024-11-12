use crate::shared::{
    protocol::world_structs::{CycleTimer, FloorMarker, SunMarker},
    shared_physics::{PhysicsBundle, FLOOR_HEIGHT, FLOOR_WIDTH},
};
use avian3d::prelude::*;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::light_consts::lux::OVERCAST_DAY;
use bevy::prelude::*;
use lightyear::client::interpolation::*;
use lightyear::shared::replication::components::Replicated;
use lightyear::{client::components::Confirmed, prelude::client::Predicted};

/// Anything correlated to general physics should be placed in this pluign
pub struct PhysicalWorldPlugin;

impl Plugin for PhysicalWorldPlugin {
    fn build(&self, app: &mut App) {
        // Set up visual interp plugins for Position and Rotation. This doesn't
        // do anything until you add VisualInterpolationStatus components to
        // entities.
        app.add_plugins(VisualInterpolationPlugin::<Position>::default());
        app.add_plugins(VisualInterpolationPlugin::<Rotation>::default());
        app.observe(add_visual_interpolation_components::<Position>);
        app.observe(add_visual_interpolation_components::<Rotation>);

        app.add_systems(Update, add_cosmetic_physics_floor);
        app.add_systems(Update, add_cosmetic_sun);
        app.add_systems(Update, daylight_cycle);
    }
}

/// This guy will add visual interpolation component to anyone that is not confirmed. or predicted
/// Basically also made to avoid stuttering
fn add_visual_interpolation_components<T: Component>(
    trigger: Trigger<OnAdd, T>,
    query: Query<
        Entity,
        (
            With<T>,
            With<Predicted>,
            Without<Confirmed>,
            Without<FloorMarker>,
        ),
    >,
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
        commands
            .entity(floor)
            .insert(PhysicsBundle::floor())
            .insert(PbrBundle {
                mesh: meshes.add(Cuboid::new(FLOOR_WIDTH, FLOOR_HEIGHT, FLOOR_WIDTH)),
                material: materials.add(Color::srgb(1.0, 1.0, 1.0)),
                ..default()
            });
    }
}

/// Will add lighting to the sun
fn add_cosmetic_sun(
    suns: Query<Entity, (Added<SunMarker>, With<SunMarker>)>,
    mut commands: Commands,
) {
    for sun in suns.iter() {
        let sun_light = DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: light_consts::lux::OVERCAST_DAY,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 2.0, 0.0),
                rotation: Quat::from_rotation_x(-std::f32::consts::PI / 4.),
                ..default()
            },
            cascade_shadow_config: CascadeShadowConfigBuilder {
                first_cascade_far_bound: 4.0,
                maximum_distance: 10.0,
                ..default()
            }
            .into(),
            ..default()
        };
        commands.entity(sun).insert(sun_light);
    }
}

fn daylight_cycle(
    mut query: Query<&mut DirectionalLight, With<SunMarker>>,
    timer: Res<CycleTimer>,
    time: Res<Time>,
) {
    if timer.0.finished() {
        let t = time.elapsed_seconds_wrapped() / 2.0;
        info!("Sun should move");
        if let Some(mut directional) = query.single_mut().into() {
            // light_trans.rotation = Quat::from_rotation_x(-t);
            directional.illuminance = t.sin().max(0.0).powf(2.0) * OVERCAST_DAY;
        }
    }
}
