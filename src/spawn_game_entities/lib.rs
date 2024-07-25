use bevy::prelude::*;


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
