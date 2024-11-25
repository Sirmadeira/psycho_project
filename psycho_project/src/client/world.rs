use crate::shared::{
    protocol::world_structs::{CycleTimer, FloorMarker},
    shared_physics::{PhysicsBundle, FLOOR_HEIGHT, FLOOR_WIDTH},
};
use avian3d::prelude::*;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::{prelude::*, render::view::NoFrustumCulling};
use lightyear::client::interpolation::*;
use lightyear::shared::replication::components::Replicated;
use lightyear::{client::components::Confirmed, prelude::client::Predicted};

/// Anything correlated to general physics should be placed in this pluign
pub struct PhysicalWorldPlugin;

impl Plugin for PhysicalWorldPlugin {
    fn build(&self, app: &mut App) {
        // Adding replicated resource from server that defines my sun position
        app.insert_resource(CycleTimer::default());

        // Set up visual interp plugins for Position and Rotation. This doesn't
        // do anything until you add VisualInterpolationStatus components to
        // entities.
        // Systems related to physical world
        app.add_plugins(VisualInterpolationPlugin::<Position>::default());
        app.add_plugins(VisualInterpolationPlugin::<Rotation>::default());
        app.observe(add_visual_interpolation_components::<Position>);
        app.observe(add_visual_interpolation_components::<Rotation>);

        // Systems related to non physical world
        app.add_systems(Startup, spawn_sun);
        app.add_systems(Update, add_cosmetic_physics_floor);
        app.add_systems(Update, orbit_around_point);
    }
}

/// A simple marker component that tells me who is the sun
#[derive(Component)]
struct SunMarker;

/// Orbit around what
const SUN_ORBIT_AROUND: Vec3 = Vec3::ZERO;
/// Radius of orbit
const SUN_RADIUS: f32 = 15.0;

const MIN_ILUMINANCE: f32 = 400.0;

const MAX_ILUMINANCE: f32 = 10000.0;

/// This guy will add visual interpolation component to anyone that is not confirmed. or predicted
/// Basically made to avoid stuttering
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
                material: materials.add(Color::srgb(0.0, 0.0, 0.0)),
                ..default()
            });
    }
}

/// Forms sun
fn spawn_sun(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
                shadows_enabled: true,
                ..default()
            },
            cascade_shadow_config: CascadeShadowConfigBuilder {
                first_cascade_far_bound: 2.0,
                maximum_distance: 20.0,
                ..default()
            }
            .into(),
            ..default()
        })
        .insert(Name::new("Sun"))
        .insert(Transform::default())
        .insert(PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::srgb(1.0, 1.0, 1.0)),
            ..default()
        })
        .insert(NoFrustumCulling)
        .insert(SunMarker);
}

/// Orbits sun
fn orbit_around_point(
    mut query: Query<(&mut Transform, &mut DirectionalLight), With<SunMarker>>,
    // time: Res<Time>,
    cycle_time: Res<CycleTimer>,
) {
    for (mut transform, mut directional_light) in query.iter_mut() {
        let cycle_fraction = cycle_time.0.elapsed_secs() / cycle_time.0.duration().as_secs_f32();

        // Calculate the max angle
        let angle = cycle_fraction * std::f32::consts::PI * 2.0;

        // Calculate the new target position using trigonometric functions
        let target_position = Vec3::new(
            -SUN_ORBIT_AROUND.x + SUN_RADIUS * angle.sin(),
            SUN_ORBIT_AROUND.y + SUN_RADIUS * angle.cos(),
            0.0,
        );

        // Smoothly interpolate between the current position and the target position
        transform.translation = transform.translation.lerp(target_position, 0.1);

        transform.look_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y);

        if angle.cos() >= 0.0 {
            // As it moves adjusts illuminations
            directional_light.illuminance =
                MIN_ILUMINANCE + (MAX_ILUMINANCE - MIN_ILUMINANCE) * angle.cos()
        } else {
            directional_light.illuminance = 0.02;
        }
    }
}
