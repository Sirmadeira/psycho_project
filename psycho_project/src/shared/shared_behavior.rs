//! Here lies every single function that should occur both to server and client. And structs for no
//! It is important to understand when you move something in client you should also try to move it in server, with the same characteristic as in client. Meaning the same input
//! As that will avoid rollbacks and mispredictions, so in summary if client input event -> apply same function -> dont do shit differently
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Probably dismantle later or dissociate but know contains all logic that is shared among client
pub struct SharedBehaviorPlugin;

impl Plugin for SharedBehaviorPlugin {
    fn build(&self, app: &mut App) {
        // Shared input systems
        // app.add_systems(Update, update_transform);
    }
}

/// Bundle that stores physic info for player characters
#[derive(Bundle)]
pub struct CharacterPhysicsBundle {
    rigid_body: RigidBody,
    collider: Collider,
    locked_axes: LockedAxes,
    // gravity: GravityScale,
    // collider_mass: ColliderMassProperties
}

impl Default for CharacterPhysicsBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::capsule(Vec3::new(0.0, 0.1, 0.0), Vec3::new(0.0, 0.5, 0.0), 0.5),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            // gravity: GravityScale(0.5),
            // collider_mass: ColliderMassProperties::Density(2.0)
        }
    }
}

/// Bundle that store physical info for my floor
#[derive(Bundle)]
pub struct FloorPhysicsBundle {
    rigid_body: RigidBody,
    collider: Collider,
}

impl Default for FloorPhysicsBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Fixed,
            collider: Collider::cuboid(100.0, 0.5, 100.0),
        }
    }
}
