//! Basically all the events associated to lobby logic
use crate::server::player::*;
use crate::shared::protocol::lobby_structs::*;
use crate::shared::protocol::player_structs::*;
use crate::shared::shared_physics::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
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
    }
}

fn replicate_resources(mut commands: Commands) {
    // Replicating resources to clients
    commands.replicate_resource::<Lobbies, Channel1>(NetworkTarget::All);
    commands.replicate_resource::<LobbyPositionMap, Channel1>(NetworkTarget::All);
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

/// Listening for clients that clicked the button start game
fn listener_join_lobby(
    mut events: EventReader<MessageEvent<EnterLobby>>,
    mut online_state: Query<&mut PlayerStateConnection>,
    mut replication_target: Query<(&mut ReplicationTarget, &mut SyncTarget)>,
    mut lobbies: ResMut<Lobbies>,
    mut lobby_position_map: ResMut<LobbyPositionMap>,
    player_entity_map: Res<PlayerEntityMap>,
    mut connection_manager: ResMut<ConnectionManager>,
    mut commands: Commands,
) {
    for event in events.read() {
        let client_id = event.context();
        let lobby_id = lobbies.lobbies[0].lobby_id;

        info!("Inserted player {} unto lobby {}", client_id, lobby_id);
        lobbies.lobbies[0].players.push(*client_id);

        let all_players = lobbies.lobbies[0].players.clone();

        info!("Mapping client position in lobby");
        lobby_position_map
            .0
            .insert(*client_id, all_players.len() - 1);

        // Adding type of replication to player who recently joined
        if let Some(player) = player_entity_map.0.get(client_id) {
            info!("New player entering game state {}", player);
            let mut on_state = online_state
                .get_mut(*player)
                .expect("For online player to have player state component");

            let (mut replication, mut sync_target) = replication_target
                .get_mut(*player)
                .expect("To pre exist replication");

            info!("Adjusting replicationg target ");
            *replication = ReplicationTarget {
                target: NetworkTarget::Only(all_players.clone()),
                ..default()
            };

            *sync_target = SyncTarget {
                prediction: NetworkTarget::Only(all_players.clone()),
                ..default()
            };

            info!("Adjusting player state to in_game");
            *on_state = PlayerStateConnection {
                online: true,
                in_game: true,
            };

            // NEVER EVER MOVE THIS GUY - THIS GUYS NEEDS TO OCCUR REALLY CLOSELY TO THE PHYSICS BUNDLE IN CLIENT
            // IF NOT HE WILL STUTTER JUST AS PLAYER SPAWNS

            commands
                .entity(*player)
                .insert(PhysicsBundle::player())
                .insert(ActionState::<CharacterAction>::default());
        }
        for all_client in lobbies.lobbies[0].players.iter() {
            if all_client != client_id {
                if let Some(other_player) = player_entity_map.0.get(all_client) {
                    let (mut replication, mut sync_target) = replication_target
                        .get_mut(*other_player)
                        .expect("To pre exist replication");

                    info!("Adjusting replicationg target ");
                    *replication = ReplicationTarget {
                        target: NetworkTarget::Only(all_players.clone()),
                        ..default()
                    };

                    *sync_target = SyncTarget {
                        prediction: NetworkTarget::Only(all_players.clone()),
                        ..default()
                    };
                }
            }
        }

        info!("Telling client id {} to start it is game", client_id);
        let _ = connection_manager
            .send_message::<Channel1, StartGame>(*client_id, &mut StartGame { lobby_id });
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

        if let Some(lobby_position) = lobby_position_map.0.remove(client_id) {
            info!("Removing client from lobby {}", client_id);
            // Safely remove the player from the `players` vector at the specified index
            if lobby_position < lobbies.lobbies[0].players.len() {
                lobbies.lobbies[0].players.remove(lobby_position);
            } else {
                warn!(
                    "Attempted to remove player at an invalid position: {}",
                    lobby_position
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
        if let Some(lobby_position) = lobby_position_map.0.remove(&client_id) {
            // Safely remove the player from the `players` vector at the specified index
            if lobby_position < lobbies.lobbies[0].players.len() {
                info!("Removing client from lobby {}", client_id);
                lobbies.lobbies[0].players.remove(lobby_position);
            } else {
                warn!(
                    "Attempted to remove player at an invalid position: {}",
                    lobby_position
                );
            }
        }
    }
}
