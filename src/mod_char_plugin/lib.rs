use bevy::prelude::*;
use bevy::utils::HashMap;

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



// Marker component that points out the entity that has and animation player
// This is handy to know who is the parent of the animation entity
#[derive(Reflect, Component, Debug)]
pub struct AnimationEntityLink(pub Entity);

// Tell me in which state the scene is
#[derive(States, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub enum StateSpawnScene {
    #[default]
    Spawning,
    Spawned,
    Done,
}

// Quick way of acessing animation data. I know  there is gltf animation but i dont want to call my asset pack all the time
#[derive(Resource)]
pub struct Animations(pub HashMap<String, Handle<AnimationClip>>);
