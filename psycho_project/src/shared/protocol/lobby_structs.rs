use bevy::prelude::*;
use bevy::utils::HashMap;
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

/// Tells me client current position in lobby
#[derive(Resource, Default, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
#[reflect(PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct LobbyPositionMap(pub HashMap<ClientId, usize>);

// Messages
/// Tells me when the game starts and what lobby to start too
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StartGame {
    pub lobby_id: u64,
}

/// Happens when someone enters lobby
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EnterLobby;

/// Happens when a player wants to leave lobbby, warn does not consider player disconnections. It  is our controlled versions
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ExitLobby;
