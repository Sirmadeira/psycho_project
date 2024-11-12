//! Basically all the events associated to lobby logic
use crate::server::player::*;
use crate::shared::protocol::lobby_structs::*;
use crate::shared::protocol::player_structs::*;

use crate::shared::shared_physics::PhysicsBundle;
use bevy::prelude::*;
use lightyear::prelude::server::*;
use lightyear::prelude::*;

/// Utilized to manage the super lobby and the lower end duel lobbies
pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        // Resources started by server
        app.init_resource::<Lobbies>();
        app.init_resource::<LobbyPositionMap>();

        // //Debugging
        // app.register_type::<Lobbies>();
        // app.register_type::<LobbyPositionMap>();

        // Replication systems
        app.add_systems(Startup, replicate_resources);
        app.add_systems(Startup, creates_major_lobby);

        // Listens to event sent by client
        app.add_systems(Update, listener_join_lobby);
        app.add_systems(Update, listener_exit_lobby);
        app.add_systems(Update, listener_disconnect_event);

        app.add_systems(FixedUpdate, insert_physics_server_player);
    }
}

fn replicate_resources(mut commands: Commands) {
    // Replicating resources to clients
    commands.replicate_resource::<Lobbies, CommonChannel>(NetworkTarget::All);
    commands.replicate_resource::<LobbyPositionMap, CommonChannel>(NetworkTarget::All);
}

/// Creates the major lobby for players also know as the white world
fn creates_major_lobby(mut lobbies: ResMut<Lobbies>) {
    let mut lobby = Lobby::default();

    info!("Grabbing lobby id");
    let lobby_id = lobbies.lobbies.len() as u64;
    lobby.lobby_id = lobby_id;

    info!("Creating lobby and replicating to others {}", lobby_id);
    lobbies.lobbies.push(lobby);
}

/// Helper patches up according to list of clietn id passed
fn update_replication_targets(
    player: Entity,
    replication_target: &mut Query<(&mut ReplicationTarget, &mut SyncTarget)>,
    all_players: &[ClientId],
) {
    info!("Updating replication targets for player {}", player);

    if let Ok((mut replication, mut sync_target)) = replication_target.get_mut(player) {
        *replication = ReplicationTarget {
            target: NetworkTarget::Only(all_players.to_vec()),
            ..Default::default()
        };

        *sync_target = SyncTarget {
            prediction: NetworkTarget::Only(all_players.to_vec()),
            ..Default::default()
        };
    } else {
        warn!(
            "Player {} is missing ReplicationTarget or SyncTarget component",
            player
        );
    }
}

fn insert_physics_server_player(
    mut events: EventReader<MessageEvent<EnterLobby>>,
    player_entity_map: Res<PlayerEntityMap>,
    mut online_state: Query<&mut PlayerStateConnection>,
    mut commands: Commands,
) {
    for event in events.read() {
        let client_id = event.context();
        if let Some(player) = player_entity_map.0.get(client_id) {
            if let Ok(mut on_state) = online_state.get_mut(*player) {
                *on_state = PlayerStateConnection {
                    online: true,
                    in_game: true,
                };
                // Insert required components for physics and action state.
                commands.entity(*player).insert(PhysicsBundle::player());
            } else {
                warn!(
                    "Player {} is missing PlayerStateConnection component",
                    player
                );
            }
        };
    }
}

/// Listening for clients that clicked the button start game - MAKE THIS AS LIGHT AS POSSIBLE
fn listener_join_lobby(
    mut events: EventReader<MessageEvent<EnterLobby>>,
    mut replication_target: Query<(&mut ReplicationTarget, &mut SyncTarget)>,
    mut lobbies: ResMut<Lobbies>,
    mut lobby_position_map: ResMut<LobbyPositionMap>,
    player_entity_map: Res<PlayerEntityMap>,
    mut connection_manager: ResMut<ConnectionManager>,
) {
    for event in events.read() {
        let client_id = event.context();
        let lobby = &mut lobbies.lobbies[0];
        let lobby_id = lobby.lobby_id;

        info!("Inserted player {} unto lobby {}", client_id, lobby_id);
        lobby.players.push(*client_id);

        let all_players = lobby.players.clone();

        // Prepare a list of all players excluding the newly joined `client_id`.
        let lobby_without_me: Vec<_> = all_players
            .iter()
            .filter(|&&player_id| player_id != *client_id)
            .copied()
            .collect();

        info!("Mapping client position in lobby");
        lobby_position_map.0.insert(
            *client_id,
            ClientInfo {
                lobby_position: all_players.len() - 1,
                lobby_without_me: lobby_without_me,
            },
        );

        for all_client in lobbies.lobbies[0].players.iter() {
            if let Some(player) = player_entity_map.0.get(all_client) {
                update_replication_targets(*player, &mut replication_target, &all_players);
            }
        }

        info!("Telling client id {} to start it is game", client_id);
        let _ = connection_manager
            .send_message::<CommonChannel, StartGame>(*client_id, &mut StartGame { lobby_id });
    }
}

/// Controlled lobby exit
fn listener_exit_lobby(
    mut events: EventReader<MessageEvent<ExitLobby>>,
    mut online_state: Query<&mut PlayerStateConnection>,
    player_entity_map: Res<PlayerEntityMap>,
    mut lobby_position_map: ResMut<LobbyPositionMap>,
    mut lobbies: ResMut<Lobbies>,
) {
    for event in events.read() {
        let client_id = event.context();

        if let Some(client_info) = lobby_position_map.0.remove(client_id) {
            info!("Removing client from lobby {}", client_id);
            // Safely remove the player from the `players` vector at the specified index
            if client_info.lobby_position < lobbies.lobbies[0].players.len() {
                lobbies.lobbies[0]
                    .players
                    .remove(client_info.lobby_position);
            } else {
                warn!(
                    "Attempted to remove player at an invalid position: {}",
                    client_info.lobby_position
                );
            }
        }

        if let Some(player_entity) = player_entity_map.0.get(client_id) {
            info!("Client disconnected but still in game {}", client_id);

            if let Ok(mut on_state) = online_state.get_mut(*player_entity) {
                *on_state = PlayerStateConnection {
                    online: true,
                    in_game: false,
                };
                info!(
                    "Manage to adjust his online state to not in game {}",
                    client_id
                )
            }
        }
    }
}

/// When disconnect from game, for any reason whatsover player is gonna be removed from lobbby
fn listener_disconnect_event(
    mut events: EventReader<DisconnectEvent>,
    mut lobby_position_map: ResMut<LobbyPositionMap>,
    mut lobbies: ResMut<Lobbies>,
) {
    for event in events.read() {
        let client_id = event.client_id;
        if let Some(client_info) = lobby_position_map.0.remove(&client_id) {
            // Safely remove the player from the `players` vector at the specified index
            if client_info.lobby_position < lobbies.lobbies[0].players.len() {
                info!("Removing client from lobby {}", client_id);
                lobbies.lobbies[0]
                    .players
                    .remove(client_info.lobby_position);
            } else {
                warn!(
                    "Attempted to remove player at an invalid position: {}",
                    client_info.lobby_position
                );
            }
        }
    }
}
