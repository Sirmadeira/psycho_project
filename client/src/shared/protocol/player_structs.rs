use std::vec;

use bevy::prelude::{Bundle, Component};
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

// Player bundle - Shared player related info important to server and client
#[derive(Bundle)]
pub(crate) struct PlayerBundle {
    id: PlayerId,
    visuals: PlayerVisuals,
}

impl PlayerBundle {
    pub(crate) fn new(id: ClientId) -> Self {
        Self {
            id: PlayerId(id),
            visuals: PlayerVisuals {
                character: vec![String::from("character_mesh.glb")],
                weapon: vec![String::from("Katana")],
            },
        }
    }
}

// Components

// Easy component that give me an easy way to acess the clientid of that specific player
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerId(pub ClientId);

// Since our character are modular we will be able to attach a series of visuals to it
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerVisuals {
    // Character related visuals - Vec of file paths
    character: Vec<String>,
    // Character weapons - Vec of weapons
    weapon: Vec<String>,
}

// Channels
#[derive(Channel)]
pub struct Channel1;
