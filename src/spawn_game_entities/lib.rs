use bevy::{prelude::*,time::Stopwatch};

// Camera
// Info for camera mechanics
#[derive(Reflect, Component, Debug)]
pub struct CamInfo {
    pub mouse_sens: f32,
    pub zoom_enabled: bool,
    pub zoom: Zoom,
    pub zoom_sens: f32,
    pub cursor_lock_activation_key: KeyCode,
    pub cursor_lock_active: bool,
}

// Sets the zoom bounds (min & max)
#[derive(Reflect, Component, Debug)]
pub struct Zoom {
    pub min: f32,
    pub max: f32,
    pub radius: f32,
}

impl Zoom {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min,
            max,
            radius: (min + max) / 2.0,
        }
    }
}

//World
// Marks ground entities
#[derive(Component)]
pub struct Ground;
// Marks wall entities
#[derive(Component)]
pub struct Wall;


// Mod char
// Tell me quantity of players
#[derive(Resource, Reflect)]
pub struct AmountPlayers {
    pub quantity: u32,
}
// Resource that defines what to add to skeleton - TODO make UI that definesi
#[derive(Resource, Reflect)]
pub struct ConfigModularCharacters {
    pub visuals_to_be_attached: Vec<String>,
    pub weapons_to_be_attached: Vec<String>,
}

// Points out visual entities and weapon entities
#[derive(Component, Reflect)]
pub struct Attachments {
    pub visual: Vec<Option<Entity>>,
    pub weapons: Vec<Option<Entity>>,
}

// Simple marker components that points out entities that can become the player
#[derive(Component)]
pub struct Skeleton;

// Marker compoenent for visual
#[derive(Component)]
pub struct Visual;

// Tell me in which state the scene is
#[derive(States, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub enum StateSpawnScene {
    #[default]
    Spawning,
    Spawned,
    Done,
}


//Player
// Marker component - Basically the rigid body that will move the player
#[derive(Component)]
pub struct Player;

// Marker just to easily check other players
#[derive(Component)]
pub struct SidePlayer;


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

// Kind of a simple pid
#[derive(Reflect, Component, Debug)]
pub struct PdInfo {
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


// Marker component - Tells me which is the collider to check for groun
#[derive(Component)]
pub struct PlayerGroundCollider;
