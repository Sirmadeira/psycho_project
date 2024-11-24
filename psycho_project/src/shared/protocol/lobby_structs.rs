use bevy::prelude::*;
use bevy::utils::HashMap;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

/// Lobby plugin related to all lobby structs
pub struct LobbyStructsPlugin;

impl Plugin for LobbyStructsPlugin {
    fn build(&self, app: &mut App) {
        //Resources
        app.register_resource::<Lobbies>(ChannelDirection::ServerToClient);
        app.register_resource::<LobbyPositionMap>(ChannelDirection::ServerToClient);

        //Messages that start start game state
        app.register_message::<StartGame>(ChannelDirection::ServerToClient);

        // Message start match related
        app.register_message::<EnterLobby>(ChannelDirection::ClientToServer);
        app.register_message::<ExitLobby>(ChannelDirection::ClientToServer);

        //Debugging
        app.register_type::<Lobbies>();
        app.register_type::<LobbyPositionMap>();
    }
}

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

/// Gives me client precious info for other logics like how would be looby without that client and what is it is position on index
#[derive(Resource, Default, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
#[reflect(Resource, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct LobbyPositionMap(pub HashMap<ClientId, ClientInfo>);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Reflect, Default)]
#[reflect(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ClientInfo {
    pub lobby_position: usize,
    pub lobby_without_me: Vec<ClientId>,
}

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
