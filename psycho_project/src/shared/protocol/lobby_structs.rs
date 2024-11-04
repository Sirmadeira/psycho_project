use bevy::prelude::*;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

// Resources
#[derive(Resource, Serialize, Deserialize, Clone, Debug, PartialEq, Default, Reflect)]
#[reflect(Resource, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Lobbies {
    pub lobbies: Vec<Lobby>,
}

// Components
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default, Reflect)]
#[reflect(PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Lobby {
    // List of lobby players
    pub players: Vec<ClientId>,
    // Identifier of lobby in list
    pub lobby_id: u64,
}
// Messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StartGame {
    pub lobby_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EnterLobby;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ExitLobby;
