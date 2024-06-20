use bevy::prelude::*;


// Marker component good to check if any of the colliders are touching the ground collider
#[derive(Reflect, Component, Debug)]
pub struct Hitbox;

// Colliders are not based on another collider axis
#[derive(Reflect, Component, Debug)]
pub struct BaseEntities {
    pub start: Entity,
    pub end: Option<Entity>,
}

// Stores the offset of the specific collider
#[derive(Reflect, Component, Debug)]
pub struct Offset(pub Vec3);

#[derive(Reflect, Component, Debug)]
pub struct PidInfo {
    // Proportional gain how agressive to reac
    pub kp: f32,
    // Integral gain accumulated error over time
    pub ki: f32,
    // Derivative gain predicts future error
    pub kd: f32,
    // These values are here because they need to be agregated
    pub integral: Vec3,
    pub previous_error: Vec3,
}