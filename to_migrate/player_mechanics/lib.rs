use bevy::utils::Duration;
use bevy::{prelude::*, time::Timer};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerSystems {
    DetectCollisions,
    KeyboardInput,
    MovePlayer,
    StatePlayer,
}

#[derive(Event, Clone, Debug)]
pub enum PlayerAction {
    Jump,
    Idle,
    Landing,
    FrontWalk,
    BackWalk,
    LeftWalk,
    RightWalk,
    FrontAir,
    BackAir,
    LeftAir,
    RightAir,
}

#[derive(Clone)]
pub struct ActionProperties {
    pub name: String,
    pub duration: Duration,
    pub repeat: bool,
}

impl ActionProperties {
    pub fn new(name: String, duration: Duration, repeat: bool) -> Self {
        Self {
            name,
            duration,
            repeat,
        }
    }
}

impl PlayerAction {
    pub fn properties(&self) -> ActionProperties {
        match self {
            PlayerAction::Idle => {
                ActionProperties::new("Idle".to_string(), Duration::from_millis(400), false)
            }
            PlayerAction::FrontWalk => {
                ActionProperties::new("FrontWalk".to_string(), Duration::from_millis(400), true)
            }
            PlayerAction::BackWalk => {
                ActionProperties::new("BackWalk".to_string(), Duration::from_millis(400), true)
            }
            PlayerAction::LeftWalk => {
                ActionProperties::new("LeftWalk".to_string(), Duration::from_millis(400), true)
            }
            PlayerAction::RightWalk => {
                ActionProperties::new("RightWalk".to_string(), Duration::from_millis(400), true)
            }
            PlayerAction::Jump => {
                ActionProperties::new("Jump".to_string(), Duration::from_millis(0), false)
            }
            PlayerAction::FrontAir => {
                ActionProperties::new("FrontAir".to_string(), Duration::from_millis(400), false)
            }
            PlayerAction::BackAir => {
                ActionProperties::new("BackAir".to_string(), Duration::from_millis(400), false)
            }
            PlayerAction::LeftAir => {
                ActionProperties::new("LeftAir".to_string(), Duration::from_millis(500), false)
            }
            PlayerAction::RightAir => {
                ActionProperties::new("RightAir".to_string(), Duration::from_millis(500), false)
            }
            PlayerAction::Landing => {
                ActionProperties::new("Landing".to_string(), Duration::from_millis(0), false)
            }
        }
    }
}

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
    pub timer: Timer,
    pub played_animation: bool,
    pub animation_name: String,
} // Example tuple with Timer and f32

impl StatusEffectDash {
    pub fn new(animation_name: String) -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            played_animation: false,
            animation_name,
        }
    }
}

#[derive(Reflect, Component, Debug)]
#[component(storage = "SparseSet")]
pub struct StatusEffectAttack {
    pub timer: Timer,
    pub played_animation: bool,
    pub animation_name: String,
}

impl StatusEffectAttack {
    pub fn new(animation_name: String) -> Self {
        Self {
            timer: Timer::from_seconds(0.8, TimerMode::Once),
            played_animation: false, // Example default value
            animation_name,
        }
    }
}

// Check if has stopped
#[derive(Reflect, Component, Debug)]
#[component(storage = "SparseSet")]
pub struct StatusIdle {
    pub timer: Timer,
}

impl Default for StatusIdle {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

// Check if it stuns - Stops animation midtrack and plays one last animation until it is finished
#[derive(Reflect, Component, Debug)]
#[component(storage = "SparseSet")]
pub struct StatusEffectStun {
    pub timer: Timer,
    pub played_animation: bool,
}
