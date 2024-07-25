use bevy::{prelude::*,time::Stopwatch};


//World
// Marks ground entities
#[derive(Component)]
pub struct Ground;
// Marks wall entities
#[derive(Component)]
pub struct Wall;


// Mod char
// Struct that will give me precious informating when assembling my character
// Leave it like this them we separate quantity

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

// Transform utilized to offset visuals, usefull to reflect helpfull functions in  blender

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