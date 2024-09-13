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
    /// If true, the lobby is in game. If not, it is still in lobby mode
    pub in_game: bool,
}

// Messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StartGame {
    pub lobby_id: usize,
}
