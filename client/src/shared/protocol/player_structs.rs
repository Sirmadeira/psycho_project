use std::vec;

use bevy::{
    prelude::{Bundle, Component, Resource},
    reflect::Reflect,
    utils::HashMap,
};
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

//Resources
// A map utilized to easily grab player info via it is client id. Avoids iterating through playerid when unecessary
#[derive(Resource, Serialize, Deserialize, Clone, Debug, PartialEq, Default, Reflect)]
pub struct PlayerBundleMap(HashMap<ClientId, PlayerBundle>);

// Player bundle - Shared player related info important to server and client
#[derive(Bundle, Serialize, Deserialize, Reflect, Clone, Debug, PartialEq)]
pub struct PlayerBundle {
    id: PlayerId,
    visuals: PlayerVisuals,
}

impl PlayerBundle {
    pub fn new(id: ClientId) -> Self {
        Self {
            id: PlayerId(id),
            visuals: PlayerVisuals::default(),
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
    // Character weapons - Vec of weapons file paths
    pub weapon: Vec<String>,
}

impl Default for PlayerVisuals {
    fn default() -> Self {
        Self {
            character: String::from("characters/character_mesh.glb"),
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
