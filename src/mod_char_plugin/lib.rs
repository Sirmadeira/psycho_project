use bevy::prelude::*;

// Struct that will give me precious informating when assembling my character
// Leave it like this them we separate quantity

#[derive(Resource,Reflect)]
pub struct AmountPlayers{
    pub quantity: u8,
}

#[derive(Resource, Reflect)]
pub struct ConfigModularCharacters {
    pub visuals_to_be_attached: Vec<String>,
    pub weapons_to_be_attached: Vec<String>,
}

#[derive(Component,Reflect)]
pub struct Attachments{
    pub visual: Vec<Option<Entity>>,
    pub weapons: Vec<Option<Entity>>
}