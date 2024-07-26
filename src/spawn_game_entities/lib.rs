use bevy::{prelude::*,time::Stopwatch,utils::HashMap};

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


//Hitboxes
// Marker component good to check if any of the colliders are touching the ground collider
#[derive(Reflect, Component, Debug)]
pub struct Hitbox;

// Marker collider that point out the collider that actually can deal damage. And to handle specific scenarios
#[derive(Component, Debug)]
pub struct WeaponCollider;

// Tell me who is the main father of the entity
#[derive(Component, Debug)]
pub struct BaseSkeleton(pub Entity);

// Component that tells me who the collider is following
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


// This is a resource, that I am gonna use to have easy acess to the info of my animation graphs
#[derive(Resource, Reflect)]
pub struct Animations {
    pub named_nodes: HashMap<String, AnimationNodeIndex>,
    pub animation_graph: Handle<AnimationGraph>,
}