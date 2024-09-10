use crate::server::spawn_player_entity;
use crate::shared::protocol::lobby_structs::*;
use crate::shared::protocol::player_structs::Channel1;
use bevy::prelude::*;
use lightyear::prelude::server::*;
use lightyear::prelude::*;

/// A client has joined a lobby:
/// - update the `Lobbies` resource
/// - add the Client to the room corresponding to the lobby
pub(crate) fn handle_lobby_join(
    mut events: EventReader<MessageEvent<JoinLobby>>,
    mut lobbies: ResMut<Lobbies>,
    mut room_manager: ResMut<RoomManager>,
    mut commands: Commands,
) {
    for lobby_join in events.read() {
        let client_id = *lobby_join.context();
        let lobby_id = lobby_join.message().lobby_id;
        info!("Client {client_id:?} joined lobby {lobby_id:?}");
        let lobby = lobbies.lobbies.get_mut(lobby_id).unwrap();
        lobby.players.push(client_id);
        room_manager.add_client(client_id, RoomId(lobby_id as u64));
        if lobby.in_game {
            // if the game has already started, we need to spawn the player entity
            let entity = spawn_player_entity(&mut commands, client_id, true);
            room_manager.add_entity(entity, RoomId(lobby_id as u64));
        }
    }
    // always make sure that there is an empty lobby for players to join
    if !lobbies.has_empty_lobby() {
        lobbies.lobbies.push(Lobby::default());
    }
}

/// A client has exited a lobby:
/// - update the `Lobbies` resource
/// - remove the Client from the room corresponding to the lobby
pub(crate) fn handle_lobby_exit(
    mut events: EventReader<MessageEvent<ExitLobby>>,
    mut lobbies: ResMut<Lobbies>,
    mut room_manager: ResMut<RoomManager>,
) {
    for lobby_join in events.read() {
        let client_id = lobby_join.context();
        let lobby_id = lobby_join.message().lobby_id;
        room_manager.remove_client(*client_id, RoomId(lobby_id as u64));
        lobbies.remove_client(*client_id);
    }
}

/// The game starts; if the host of the game is the dedicated server, we will spawn a cube
/// for each player in the lobby
pub(crate) fn handle_start_game(
    mut connection_manager: ResMut<ConnectionManager>,
    mut events: EventReader<MessageEvent<StartGame>>,
    mut lobbies: ResMut<Lobbies>,
    mut room_manager: ResMut<RoomManager>,
    mut commands: Commands,
) {
    for event in events.read() {
        let client_id = event.context();
        let lobby_id = event.message().lobby_id;
        let host = event.message().host;
        let lobby = lobbies.lobbies.get_mut(lobby_id).unwrap();

        // Setting lobby ingame
        if !lobby.in_game {
            lobby.in_game = true;
            if let Some(host) = host {
                lobby.host = Some(host);
            }
        }

        let room_id = RoomId(lobby_id as u64);
        // the client was not part of the lobby, they are joining in the middle of the game
        if !lobby.players.contains(client_id) {
            lobby.players.push(*client_id);
            if host.is_none() {
                let entity = spawn_player_entity(&mut commands, *client_id, true);
                room_manager.add_entity(entity, room_id);
                room_manager.add_client(*client_id, room_id);
            }
            // send the StartGame message to the client who is trying to join the game
            let _ = connection_manager.send_message::<Channel1, _>(
                *client_id,
                &mut StartGame {
                    lobby_id,
                    host: lobby.host,
                },
            );
        } else {
            if host.is_none() {
                // one of the players asked for the game to start
                for player in &lobby.players {
                    info!("Spawning player  {player:?} in server hosted  game");
                    let entity = spawn_player_entity(&mut commands, *player, true);
                    room_manager.add_entity(entity, room_id);
                }
            }
            // redirect the StartGame message to all other clients in the lobby
            let _ = connection_manager.send_message_to_target::<Channel1, _>(
                &mut StartGame {
                    lobby_id,
                    host: lobby.host,
                },
                NetworkTarget::Only(lobby.players.clone()),
            );
        }
    }
}
