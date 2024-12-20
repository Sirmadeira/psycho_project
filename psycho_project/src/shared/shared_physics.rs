//! Here lies every single function that should occur both to server and client.
//! It is important to understand when you move something in client you should also try to move it in server, with the same characteristic as in client. Meaning the same input
//! As that will avoid rollbacks and mispredictions, so in summary if client input event -> apply same function -> dont do shit differently
use crate::shared::protocol::player_structs::*;
use avian3d::prelude::*;
use avian3d::sync::SyncConfig;
use bevy::ecs::query::QueryData;
use bevy::prelude::*;
use common::shared::FIXED_TIMESTEP_HZ;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::ReplicationGroup;
/// Here lies all the shared setup needed to make physics work in our game
/// Warning: This game is solely based on running an independent server and clients any other mode will break it
pub struct SharedPhysicsPlugin;

impl Plugin for SharedPhysicsPlugin {
    fn build(&self, app: &mut App) {
        // SETUP FOR MAKING OUR PHYSICS WORK
        app.add_plugins(
            PhysicsPlugins::new(FixedUpdate)
                .build()
                .disable::<SyncPlugin>()
                .disable::<SleepingPlugin>(),
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
        // app.add_plugins(PhysicsDebugPlugin::new(PostUpdate));

        //Our shared gravity
        app.insert_resource(Gravity(Vec3::new(0.0, -10.0, 0.0)));

        // Disabling warm collision = TODO MAKE IT ROLLBACKABLE
        app.insert_resource(NarrowPhaseConfig {
            match_contacts: false,
            ..default()
        });

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

#[derive(Bundle)]
pub struct PlayerPhysics {
    pub collider: Collider,
    pub collider_density: ColliderDensity,
    pub rigid_body: RigidBody,
    pub external_force: ExternalForce,
    pub locked_axes: LockedAxes,
    pub collison_layer: CollisionLayers,
    pub friction: Friction,
    pub grounded_caster: RayCaster,
}

impl Default for PlayerPhysics {
    fn default() -> Self {
        let collider = Collider::capsule(CHARACTER_CAPSULE_RADIUS, CHARACTER_CAPSULE_HEIGHT);
        Self {
            collider,
            collider_density: ColliderDensity(1.0),
            locked_axes: LockedAxes::default().lock_rotation_x().lock_rotation_z(),
            rigid_body: RigidBody::Dynamic,
            external_force: ExternalForce::ZERO.with_persistence(false),
            collison_layer: CollisionLayers::new(
                GameLayer::Player,
                [GameLayer::Ground, GameLayer::Bullet],
            ),
            friction: Friction::new(0.3).with_combine_rule(CoefficientCombine::Min),
            grounded_caster: RayCaster::new(
                Vec3::new(
                    0.0,
                    -CHARACTER_CAPSULE_HEIGHT / 2.0 - CHARACTER_CAPSULE_RADIUS,
                    0.0,
                ),
                Dir3::NEG_Y,
            )
            .with_max_time_of_impact(0.2)
            .with_solidness(true),
        }
    }
}

#[derive(Bundle)]
pub struct FloorPhysics {
    pub collider: Collider,
    pub rigid_body: RigidBody,
}

impl Default for FloorPhysics {
    fn default() -> Self {
        Self {
            collider: Collider::cuboid(FLOOR_WIDTH, FLOOR_HEIGHT, FLOOR_WIDTH),
            rigid_body: RigidBody::Static,
        }
    }
}

#[derive(Bundle)]
pub struct BulletPhysics {
    pub collider: Collider,
    pub collider_density: ColliderDensity,
    pub rigid_body: RigidBody,
    pub external_force: ExternalForce,
    pub collison_layer: CollisionLayers,
}

impl Default for BulletPhysics {
    fn default() -> Self {
        Self {
            collider: Collider::cylinder(BULLET_RADIUS, BULLET_HEIGHT),
            collider_density: ColliderDensity(1.0),
            rigid_body: RigidBody::Dynamic,
            external_force: ExternalForce::default(),
            collison_layer: CollisionLayers::new(
                GameLayer::Bullet,
                [GameLayer::Player, GameLayer::Ground],
            ),
        }
    }
}

#[derive(QueryData)]
#[query_data(mutable, derive(Debug))]
pub struct CharacterQuery {
    pub external_force: &'static mut ExternalForce,
    pub external_impulse: &'static mut ExternalImpulse,
    pub linear_velocity: &'static LinearVelocity,
    pub angular_velocity: &'static AngularVelocity,
    pub mass: &'static Mass,
    pub position: &'static Position,
    pub rotation: &'static mut Rotation,
    pub entity: Entity,
}

/// Apply the character actions `action_state` to the character entity `character`.
/// TODO - FIX THIS SO IT FILTER OUT ALL OTHER CHARACTERS
pub fn apply_character_action(
    time: &Res<Time>,
    action_state: &ActionState<PlayerAction>,
    ray_hits: &RayHits,
    character: &mut CharacterQueryItem,
) {
    const MAX_SPEED: f32 = 5.0;
    const MAX_ACCELERATION: f32 = 20.0;
    // How much velocity can change in a single tick given the max acceleration.
    let max_velocity_delta_per_tick = MAX_ACCELERATION * time.delta_seconds();

    // Handle jumping.
    if action_state.just_pressed(&PlayerAction::Jump) {
        if !ray_hits.is_empty() {
            character
                .external_impulse
                .apply_impulse(Vec3::new(0.0, 5.0, 0.0));
        }
    }

    let move_dir = action_state
        .axis_pair(&PlayerAction::Direction)
        .clamp_length_max(1.0);

    // Handle moving.
    let move_dir = Vec3::new(move_dir.x, 0.0, move_dir.y);

    // Linear velocity of the character ignoring vertical speed.
    let ground_linear_velocity = Vec3::new(
        character.linear_velocity.x,
        0.0,
        character.linear_velocity.z,
    );

    let desired_ground_linear_velocity = move_dir * MAX_SPEED;

    let new_ground_linear_velocity = ground_linear_velocity
        .move_towards(desired_ground_linear_velocity, max_velocity_delta_per_tick);

    let required_acceleration =
        (new_ground_linear_velocity - ground_linear_velocity) / time.delta_seconds();

    if move_dir.length_squared() != 0.0 {
        character
            .external_force
            .apply_force(required_acceleration * character.mass.0);
    }

    // Apply gravity

    // character
    //     .external_force
    //     .apply_force(Vec3::new(0.0, -10.0, 0.0));

    // Handle looking at

    // let camera_rotation = action_state.axis_pair(&PlayerAction::RotateToCamera);

    // let quat_rotation = Quat::from_euler(EulerRot::YXZ, camera_rotation.y, 0.0, 0.0);

    // character.rotation.0 = quat_rotation;
}
