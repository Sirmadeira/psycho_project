use bevy::{prelude::*, time::Timer};

#[derive(Event, Debug)]
pub enum MovementAction {
    // Movement direction
    Move(Vec2),
    // Dash direction
    Dash(Vec2),
    // Jump status
    Jump,
}
// Will tell me which type of attack I should do, I use this to avoid conflicts with animation
#[derive(Event, Debug)]
pub enum TypeOfAttack {
    None,
    Forward,
    Backward,
    Left,
    Right,
}

#[derive(Event, Debug)]
pub enum RotateAction {
    EaseRotation(Vec3),
}

// Check if is on ground
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

// Checks if has wallbounced
#[derive(Reflect, Component, Debug)]
#[component(storage = "SparseSet")]
pub struct StatusEffectWallBounce {
    pub bounce_duration: Timer,
}

// Check if has dashed
#[derive(Reflect, Component, Debug)]
#[component(storage = "SparseSet")]
pub struct StatusEffectDash {
    pub dash_cooldown: Timer,
}

// Check if has status defend
#[derive(Reflect, Component, Debug)]
#[component(storage = "SparseSet")]
pub struct StatusEffectDefend {
    pub dash_cooldown: Timer,
}

// Check if has stopped
#[derive(Reflect, Component, Debug)]
#[component(storage = "SparseSet")]
pub struct StatusIdle;


