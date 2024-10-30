//! Here lies every single function that should occur both to server and client. And structs for no
//! It is important to understand when you move something in client you should also try to move it in server, with the same characteristic as in client. Meaning the same input
//! As that will avoid rollbacks and mispredictions, so in summary if client input event -> apply same function -> dont do shit differently
use avian3d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::ReplicationGroup;

/// Probably dismantle later or dissociate but know contains all logic that is shared among client
pub struct SharedBehaviorPlugin;

impl Plugin for SharedBehaviorPlugin {
    fn build(&self, app: &mut App) {
        // Shared input systems
        // app.add_systems(Update, update_transform);
    }
}

pub const REPLICATION_GROUP: ReplicationGroup = ReplicationGroup::new_id(1);

pub const CHARACTER_CAPSULE_RADIUS: f32 = 0.5;
pub const CHARACTER_CAPSULE_HEIGHT: f32 = 0.5;

/// Bundle that stores physical info for my character
#[derive(Bundle)]
pub struct CharacterPhysicsBundle {
    rigid_body: RigidBody,
    collider: Collider,
    lock_axes: LockedAxes,
    friction: Friction,
}

impl Default for CharacterPhysicsBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::capsule(CHARACTER_CAPSULE_RADIUS, CHARACTER_CAPSULE_HEIGHT),
            lock_axes: LockedAxes::default()
                .lock_rotation_x()
                .lock_rotation_y()
                .lock_rotation_z(),
            friction: Friction::new(0.0).with_combine_rule(CoefficientCombine::Min),
        }
    }
}

pub const FLOOR_WIDTH: f32 = 100.0;
pub const FLOOR_HEIGHT: f32 = 0.5;

/// Bundle that store physical info for my floor
#[derive(Bundle)]
pub struct FloorPhysicsBundle {
    rigid_body: RigidBody,
    collider: Collider,
    position: Position,
}

impl Default for FloorPhysicsBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Static,
            collider: Collider::cuboid(FLOOR_WIDTH, FLOOR_HEIGHT, FLOOR_WIDTH),
            position: Position(Vec3::new(0.0, -0.5, 0.0)),
        }
    }
}
