use bevy::prelude::{Bundle, Component};
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

// Player bundler - Create the matrix of our player it give me a serious of needed info
#[derive(Bundle)]
pub(crate) struct PlayerBundle {
    id: PlayerId,
}

impl PlayerBundle {
    pub(crate) fn new(id: ClientId) -> Self {
        Self { id: PlayerId(id) }
    }
}

// Components

// Marker component tells me exactly who connected, which player and so on. Via it is id also know as ip
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerId(pub ClientId);

// Channels
#[derive(Channel)]
pub struct Channel1;
