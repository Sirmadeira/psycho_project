use bevy::prelude::*;
use std::collections::HashMap;

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

// This is a resource, that I am gonna use to have easy acess to the info of my animation graphs
#[derive(Resource,Reflect)]
pub struct Animations{
    pub named_nodes: HashMap<String,AnimationNodeIndex>,
    pub animation_graph: Handle<AnimationGraph>
}



// Simple marker components that points out entities that can become the player
#[derive(Component)]
pub struct Skeleton;


// Tell me in which state the scene is
#[derive(States, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub enum StateSpawnScene {
    #[default]
    Spawning,
    Spawned,
    Done,
}

