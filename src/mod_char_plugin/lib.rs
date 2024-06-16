use bevy::prelude::*;

// Struct that will give me precious informating when assembling my character
// Leave it like this them we separate quantity
#[derive(Resource, Reflect)]
pub struct ConfigModularCharacters {
    pub quantity: u8,
    pub visuals_to_be_attached: Vec<String>,
    pub weapons: Vec<String>,
}
