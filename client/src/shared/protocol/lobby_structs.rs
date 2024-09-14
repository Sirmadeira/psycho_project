use bevy::prelude::*;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

// Components
#[derive(Resource, Serialize, Deserialize, Clone, Debug, PartialEq, Default, Reflect)]
pub struct Lobbies {
    pub lobbies: Vec<Lobby>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default, Reflect)]
pub struct Lobby {
    // List of lobby players
    pub players: Vec<ClientId>,
    // Identifier of lobby in list
    pub lobby_id: usize,
}
// Messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StartGame {
    pub lobby_id: usize,
}