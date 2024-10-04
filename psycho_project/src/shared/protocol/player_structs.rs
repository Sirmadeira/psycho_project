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

// Visuals of our character
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
    // Character available weapon - Vec of weapons file paths
    pub weapon_1: String,
    // Also know as the "glue" of modular characters
    pub skeleton: String,
}

impl Default for PlayerVisuals {
    fn default() -> Self {
        Self {
            character: String::from("characters/character_mesh.glb"),
            head: String::from("characters/parts/suit_head.glb"),
            torso: String::from("characters/parts/scifi_torso.glb"),
            legs: String::from("characters/parts/witch_legs.glb"),
            weapon_1: String::from("weapons/katana.glb"),
            skeleton: String::from("characters/parts/main_skeleton.glb"),
        }
    }
}

impl PlayerVisuals {
    // Returns an iterator over the visual components. Inclu
    pub fn iter_visuals(&self) -> impl Iterator<Item = &String> {
        vec![&self.head, &self.torso, &self.legs, &self.skeleton].into_iter()
    }
}

// Channels
#[derive(Channel)]
pub struct Channel1;

// Messages
// An event message sent by server to give a recently loaded client it is bundle
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SendBundle(pub PlayerBundle);

// An event message sent by client to server that gives the player currently chosen loadout
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SaveVisual(pub PlayerVisuals, pub PartToChange);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default, Reflect)]
#[reflect(PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct PartToChange {
    // File path to old part
    pub old_part: String,
    // File Path to new part
    pub new_part: String,
}

// AN event message sent by server to client that tells me that player can customize his character
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ChangeChar(pub PartToChange);
