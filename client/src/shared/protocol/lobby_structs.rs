use bevy::prelude::*;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

// All lobbies currently active
#[derive(Resource, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
pub struct Lobbies {
    pub lobbies: Vec<Lobby>,
}

impl Default for Lobbies {
    fn default() -> Self {
        Self {
            lobbies: vec![Lobby::default()],
        }
    }
}

impl Lobbies {
    /// Return true if there is an empty lobby available for players to join
    pub(crate) fn has_empty_lobby(&self) -> bool {
        // Scenario where lobby is empty
        if self.lobbies.is_empty() {
            return false;
        }
        // Returns true if any of the lobbies has at least one player
        self.lobbies.iter().any(|lobby| lobby.players.is_empty())
    }

    /// Remove a player from lobby if there is no player delete it. Also clears empty lobby. Also creates defaul lobby
    pub(crate) fn remove_client(&mut self, client_id: ClientId) {
        let mut removed_lobby = None;
        // Number of lobby
        for (lobby_id, lobby) in self.lobbies.iter_mut().enumerate() {
            if let Some(index) = lobby.players.iter().position(|id| *id == client_id) {
                lobby.players.remove(index);
                if lobby.players.is_empty() {
                    removed_lobby = Some(lobby_id);
                }
            }
        }
        if let Some(lobby_id) = removed_lobby {
            self.lobbies.remove(lobby_id);
            if !self.has_empty_lobby() {
                self.lobbies.push(Lobby::default());
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default, Reflect)]
pub struct Lobby {
    // List of lobby players
    pub players: Vec<ClientId>,
    /// Which client is selected to be the host for the next game (if None, the server will be the host)
    pub host: Option<ClientId>,
    /// If true, the lobby is in game. If not, it is still in lobby mode
    pub in_game: bool,
}

// Messages

// Tells me when a game has started
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StartGame {
    // Lobby id
    pub(crate) lobby_id: usize,
    // Current host
    pub(crate) host: Option<ClientId>,
}

// Tells me when someone exits lobby
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ExitLobby {
    pub(crate) lobby_id: usize,
}
// Tells me when someone joins lobby
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct JoinLobby {
    pub(crate) lobby_id: usize,
}
