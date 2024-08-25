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

// Check if is on ground
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;


// Check if has dashed

#[derive(Reflect, Component, Debug)]
#[component(storage = "SparseSet")]
pub struct StatusEffectDash(pub Timer); // Example tuple with Timer and f32

impl Default for StatusEffectDash {
    fn default() -> Self {
        StatusEffectDash(Timer::from_seconds(1.0, TimerMode::Once)) // Example default values
    }
}


// Check if has status defend
#[derive(Reflect, Component, Debug)]
#[component(storage = "SparseSet")]
pub struct StatusEffectDefend {
    pub dash_cooldown: Timer,
}

#[derive(Reflect, Component, Debug)]
#[component(storage = "SparseSet")]
pub struct StatusEffectAttack(pub Timer);

impl Default for StatusEffectAttack {
    fn default() -> Self {
        StatusEffectAttack(Timer::from_seconds(1.0, TimerMode::Once))  // Example default value
    }
}

// Check if has stopped
#[derive(Reflect, Component, Debug)]
#[component(storage = "SparseSet")]
pub struct StatusIdle(pub Timer);

// Check if it stuns - Stops animation midtrack and plays one last animation until it is finished
#[derive(Reflect, Component, Debug)]
#[component(storage = "SparseSet")]
pub struct StatusEffectStun {
    pub timer: Timer,
    pub played_animation: bool,
}
