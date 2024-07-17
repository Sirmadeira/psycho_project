use bevy::{
    prelude::*,
    time::{Stopwatch, Timer},
};

// Marker component - Basically the rigid body that will move the player points to it is skeleton
#[derive(Component)]
pub struct Player;

// Skeletons that are yet to have a player point to it is skeleton 
#[derive(Component)]
pub struct SidePlayer;

// Marker component -
#[derive(Component)]
pub struct PlayerGroundCollider;

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

// Kind of a simple pid
#[derive(Reflect, Component, Debug)]
pub struct PdInfo {
    // Proportional gain how agressive to reac
    pub kp: f32,
}

// Times the dash for each key
#[derive(Reflect, Component, Debug)]
pub struct Timers {
    pub up: Stopwatch,
    pub down: Stopwatch,
    pub left: Stopwatch,
    pub right: Stopwatch,
}

// Amount of jumps you can have
#[derive(Reflect, Component, Debug)]
pub struct Limit {
    pub jump_limit: u8,
}

impl Default for Limit {
    fn default() -> Self {
        Self { jump_limit: 2 }
    }
}

#[derive(Component, Reflect, Debug)]

pub struct Health(pub i8);
