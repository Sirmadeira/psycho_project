use bevy::{
    prelude::*,
    time::{Stopwatch, Timer},
};

#[derive(States, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub enum StatePlayerCreation {
    #[default]
    Spawning,
    Done,
}

// Marker component - Basically the rigid body that will move the player
#[derive(Component)]
pub struct Player;

// Skeletons that are yet to have a player
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

// Check if is on ground
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

// Check if has dashed
#[derive(Reflect, Component, Debug)]
#[component(storage = "SparseSet")]
pub struct StatusEffectDash {
    pub dash_duration: Timer,
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
