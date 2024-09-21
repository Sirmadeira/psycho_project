use std::vec;

use bevy::prelude::*;
use bevy::{reflect::Reflect, utils::HashMap};
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

//Resources
// A map utilized to easily grab player info via it is client id. Avoids iterating through playerid when unecessary
#[derive(Resource, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect, Default)]
#[reflect(Resource, PartialEq, Debug, Serialize, Deserialize)]
pub struct PlayerBundleMap(pub HashMap<ClientId, PlayerBundle>);

// Player bundle - Shared player related info important to server and client
#[derive(Bundle, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
#[reflect(PartialEq, Debug, Serialize, Deserialize)]
pub struct PlayerBundle {
    pub id: PlayerId,
    pub visuals: PlayerVisuals,
}

impl PlayerBundle {
    pub fn new(id: ClientId, visuals: PlayerVisuals) -> Self {
        Self {
            id: PlayerId(id),
            visuals: visuals,
        }
    }
}

// Components

// Easy component that give me an easy way to acess the clientid of that specific player
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
pub struct PlayerId(pub ClientId);

// Since our character are modular we will be able to attach a series of visuals to it
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
pub struct PlayerVisuals {
    // Character related visuals - Vec of file paths
    pub character: String,
    // Character head visuals - Vec of file paths
    pub head: String,
    // Character torso visuals - Vec of file paths
    pub torso: String,
    // Character torso visuals - Vec of file paths
    pub legs: String,
    // Character weapons - Vec of weapons file paths
    pub weapon: Vec<String>,
}

impl Default for PlayerVisuals {
    fn default() -> Self {
        Self {
            character: String::from("characters/character_mesh.glb"),
            head: String::from("characters/mod_char/farmer_head.glb"),
            torso: String::from("character/mod_char/scifi_torso.glb"),
            legs: String::from("character/mod_char/witch_legs.glb"),
            weapon: vec![String::from("weapons/katana.glb")],
        }
    }
}

// Channels
#[derive(Channel)]
pub struct Channel1;

// Messages
// An event message sent by client to server that gives the player currently chosen loadout
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerLoadout(pub PlayerVisuals);
