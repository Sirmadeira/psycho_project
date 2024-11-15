//! Here lies every single function that should occur both to server and client.
//! It is important to understand when you move something in client you should also try to move it in server, with the same characteristic as in client. Meaning the same input
//! As that will avoid rollbacks and mispredictions, so in summary if client input event -> apply same function -> dont do shit differently
use crate::shared::protocol::player_structs::*;
use crate::shared::protocol::weapon_structs::*;
use avian3d::prelude::*;
use avian3d::sync::SyncConfig;
use bevy::ecs::query::QueryData;
use bevy::prelude::*;
use common::shared::FIXED_TIMESTEP_HZ;
use leafwing_input_manager::prelude::*;
use lightyear::client::prediction::prespawn::PreSpawnedPlayerObject;
use lightyear::prelude::client::Predicted;
use lightyear::prelude::server::Replicate;
use lightyear::prelude::server::SyncTarget;
use lightyear::prelude::{ReplicationGroup, ReplicationTarget};
use lightyear::shared::plugin::NetworkIdentity;
use lightyear::shared::replication::network_target::NetworkTarget;
use lightyear::shared::tick_manager::TickManager;

use super::protocol::lobby_structs::Lobbies;
/// Here lies all the shared setup needed to make physics work in our game
/// Warning: This game is solely based on running an independent server and clients any other mode will break it
pub struct SharedPhysicsPlugin;

impl Plugin for SharedPhysicsPlugin {
    fn build(&self, app: &mut App) {
        // SETUP FOR MAKING OUR PHYSICS WORK
        app.add_plugins(
            PhysicsPlugins::new(FixedUpdate)
                .build()
                .disable::<SyncPlugin>(),
        );
        // We change SyncPlugin to PostUpdate, because we want the visually
        // interpreted values synced to transform every time, not just when
        // Fixed schedule runs.
        app.add_plugins(SyncPlugin::new(PostUpdate));
        // Position and Rotation are the primary source of truth so no need to
        // sync changes from Transform to Position.
        app.insert_resource(SyncConfig {
            transform_to_position: false,
            position_to_transform: true,
        });
        // Setting timestep to same rate as fixed timestep hz it is 74 btw
        app.insert_resource(Time::new_with(Physics::fixed_once_hz(FIXED_TIMESTEP_HZ)));

        // See your colliders
        app.add_plugins(PhysicsDebugPlugin::default());

        // Setting up gravity - NEED TO BE ZERO, OR ELSE jiter
        app.insert_resource(Gravity(Vec3::new(0.0, -2.0, 0.0)));

        // Make sure that any physics simulation happens after the input
        app.configure_sets(
            FixedUpdate,
            (
                (
                    PhysicsSet::Prepare,
                    PhysicsSet::StepSimulation,
                    PhysicsSet::Sync,
                )
                    .in_set(InputPhysicsSet::Physics),
                (InputPhysicsSet::Input, InputPhysicsSet::Physics).chain(),
            ),
        );
    }
}

/// Super important set ensures that our input systems occur before the physics, if not followed get ready for stuttering
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum InputPhysicsSet {
    // Input system sets
    Input,
    // Apply physics steps.
    Physics,
}

pub const REPLICATION_GROUP: ReplicationGroup = ReplicationGroup::new_id(1);

pub const CHARACTER_CAPSULE_RADIUS: f32 = 0.5;
pub const CHARACTER_CAPSULE_HEIGHT: f32 = 0.5;

pub const FLOOR_WIDTH: f32 = 100.0;
pub const FLOOR_HEIGHT: f32 = 0.5;

pub const BULLET_RADIUS: f32 = 0.5;
pub const BULLET_HEIGHT: f32 = 0.5;

/// Collision layers
#[derive(PhysicsLayer)]
enum GameLayer {
    Player,
    Ground,
    Bullet,
}

/// Physics bundle, subdivided according to necessity
#[derive(Bundle)]
pub struct PhysicsBundle {
    pub collider: Collider,
    pub collider_density: ColliderDensity,
    pub rigid_body: RigidBody,
    pub external_force: ExternalForce,
    pub locked_axes: LockedAxes,
    pub collison_layer: CollisionLayers,
    pub friction: Friction,
}

impl PhysicsBundle {
    pub fn player() -> Self {
        let collider = Collider::capsule(CHARACTER_CAPSULE_RADIUS, CHARACTER_CAPSULE_HEIGHT);
        Self {
            collider,
            collider_density: ColliderDensity(1.0),
            locked_axes: LockedAxes::default()
                .lock_rotation_x()
                .lock_rotation_y()
                .lock_rotation_z(),
            rigid_body: RigidBody::Dynamic,
            external_force: ExternalForce::ZERO.with_persistence(false),
            collison_layer: CollisionLayers::new(GameLayer::Player, [GameLayer::Ground]),
            friction: Friction::new(0.0).with_combine_rule(CoefficientCombine::Min),
        }
    }
    pub fn floor() -> Self {
        Self {
            collider: Collider::cuboid(FLOOR_WIDTH, FLOOR_HEIGHT, FLOOR_WIDTH),
            collider_density: ColliderDensity(1.0),
            rigid_body: RigidBody::Static,
            external_force: ExternalForce::ZERO.with_persistence(false),
            locked_axes: LockedAxes::default(),
            collison_layer: CollisionLayers::new(
                GameLayer::Ground,
                [GameLayer::Ground, GameLayer::Player],
            ),
            friction: Friction::new(0.0).with_combine_rule(CoefficientCombine::Min),
        }
    }

    pub(crate) fn bullet() -> Self {
        Self {
            collider: Collider::cylinder(BULLET_RADIUS, BULLET_HEIGHT),
            collider_density: ColliderDensity(1.0),
            rigid_body: RigidBody::Dynamic,
            external_force: ExternalForce::default(),
            locked_axes: LockedAxes::default(),
            collison_layer: CollisionLayers::new(GameLayer::Bullet, [GameLayer::Player]),
            friction: Friction::new(0.0).with_combine_rule(CoefficientCombine::Min),
        }
    }
}

#[derive(QueryData)]
#[query_data(mutable, derive(Debug))]
pub struct CharacterQuery {
    pub external_force: &'static mut ExternalForce,
    pub external_impulse: &'static mut ExternalImpulse,
    pub linear_velocity: &'static LinearVelocity,
    pub mass: &'static Mass,
    pub position: &'static Position,
    pub entity: Entity,
}

/// Apply the character actions `action_state` to the character entity `character`.
pub fn apply_character_action(
    time: &Res<Time>,
    spatial_query: &SpatialQuery,
    action_state: &ActionState<CharacterAction>,
    character: &mut CharacterQueryItem,
) {
    const MAX_SPEED: f32 = 5.0;
    const MAX_ACCELERATION: f32 = 20.0;
    // How much velocity can change in a single tick given the max acceleration.
    let max_velocity_delta_per_tick = MAX_ACCELERATION * time.delta_seconds();

    // Handle jumping.
    if action_state.pressed(&CharacterAction::Jump) {
        let ray_cast_origin = character.position.0
            + Vec3::new(
                0.0,
                -CHARACTER_CAPSULE_HEIGHT / 2.0 - CHARACTER_CAPSULE_RADIUS,
                0.0,
            );

        // Only jump if the character is on the ground.
        //
        // Check if we are touching the ground by sending a ray from the bottom
        // of the character downwards.
        if spatial_query
            .cast_ray(
                ray_cast_origin,
                Dir3::NEG_Y,
                0.01,
                true,
                SpatialQueryFilter::from_excluded_entities([character.entity]),
            )
            .is_some()
        {
            character
                .external_impulse
                .apply_impulse(Vec3::new(0.0, 5.0, 0.0));
        }
    }

    // Handle moving.
    let move_dir = action_state
        .axis_pair(&CharacterAction::Move)
        .clamp_length_max(1.0);
    let move_dir = Vec3::new(-move_dir.x, 0.0, move_dir.y);

    // Linear velocity of the character ignoring vertical speed.
    let ground_linear_velocity = Vec3::new(
        character.linear_velocity.x,
        0.0,
        character.linear_velocity.z,
    );

    let desired_ground_linear_velocity = move_dir * MAX_SPEED;

    let new_ground_linear_velocity = ground_linear_velocity
        .move_towards(desired_ground_linear_velocity, max_velocity_delta_per_tick);

    // Acceleration required to change the linear velocity from
    // `ground_linear_velocity` to `new_ground_linear_velocity` in the duration
    // of a single tick.
    //
    // There is no need to clamp the acceleration's length to
    // `MAX_ACCELERATION`. The difference between `ground_linear_velocity` and
    // `new_ground_linear_velocity` is never great enough to require more than
    // `MAX_ACCELERATION` in a single tick, This is because
    // `new_ground_linear_velocity` is calculated using
    // `max_velocity_delta_per_tick` which restricts how much the velocity can
    // change in a single tick based on `MAX_ACCELERATION`.
    let required_acceleration =
        (new_ground_linear_velocity - ground_linear_velocity) / time.delta_seconds();

    character
        .external_force
        .apply_force(required_acceleration * character.mass.0);
}

// Warning - This function needs to be importable, because although client can spawn a prespawned object he should never do that in rollback
// Or else we spawn two bulletse
pub fn shared_spawn_bullet(
    mut query: Query<
        (
            &Position,
            &Rotation,
            &LinearVelocity,
            &PlayerId,
            &ActionState<CharacterAction>,
            &mut Weapon,
        ),
        Or<(With<Predicted>, With<ReplicationTarget>)>,
    >,
    tick_manager: Res<TickManager>,
    lobbies: Res<Lobbies>,
    mut commands: Commands,
    identity: NetworkIdentity,
) {
    // If there is no entity no need for this system to be enabled
    if query.is_empty() {
        return;
    }
    // Current tick
    let current_tick = tick_manager.tick();

    for (player_position, player_rotation, player_velocity, player_id, action_state, mut weapon) in
        query.iter_mut()
    {
        if !action_state.just_pressed(&CharacterAction::Shoot) {
            continue;
        }

        info!("Fired bullet");
        // Tick difference between weapon and current tick
        let tick_diff = weapon.last_fire_tick - current_tick;

        // Checking if weapon
        if tick_diff.abs() <= weapon.cooldown as i16 {
            // Here he cant technically fire for now as he is in cooldown
            if weapon.last_fire_tick == current_tick {
                info!("Player cant fire for now, as he is firing in same tick")
            }
            continue;
        }

        let prev_last_fire_tick = weapon.last_fire_tick;
        weapon.last_fire_tick = current_tick;

        let bullet_spawn_offset = Vec3::new(0.0, 1.0, 0.0);
        let bullet_origin = player_position.0 + bullet_spawn_offset;
        let bullet_linvel = player_rotation * (Vec3::Z * weapon.bullet_speed) + player_velocity.0;

        // We do this to avo
        let prespawned = PreSpawnedPlayerObject::default_with_salt(player_id.0.to_bits());

        let bullet_entity = commands
            .spawn((
                BulletBundle::new(player_id.0, bullet_origin, bullet_linvel, current_tick),
                PhysicsBundle::bullet(),
                prespawned,
            ))
            .id();
        info!(
            "spawned bullet for ActionState, bullet={bullet_entity:?} ({}, {}). prev last_fire tick: {prev_last_fire_tick:?}",
            weapon.last_fire_tick.0, player_id.0
        );
        if identity.is_server() {
            info!("Replicating bullet for others in lobbies");
            let replicate = Replicate {
                sync: SyncTarget {
                    prediction: NetworkTarget::Only(lobbies.lobbies[0].players.clone()),
                    ..Default::default()
                },
                // make sure that all entities that are predicted are part of the same replication group
                group: REPLICATION_GROUP,
                ..default()
            };
            commands.entity(bullet_entity).insert(replicate);
        }
    }
}
