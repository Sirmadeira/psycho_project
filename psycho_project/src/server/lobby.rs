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
        //Debugging
        app.register_type::<Lobbies>();

        // Replication systems
        app.add_systems(Startup, replicate_resource);
        app.add_systems(Startup, create_lobby);

        // Listens to event sent by client
        app.add_systems(Update, listener_join_lobby);
        app.add_systems(Update, listener_exit_lobby);
    }
}

fn replicate_resource(mut commands: Commands) {
    // Replicating resources to clients
    commands.replicate_resource::<Lobbies, Channel1>(NetworkTarget::All);
}

/// Creates the major lobby for players also know as the white world
fn create_lobby(mut lobbies: ResMut<Lobbies>) {
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
    mut lobbies: ResMut<Lobbies>,
    player_entity_map: Res<PlayerEntityMap>,
    mut online_state: Query<&mut PlayerStateConnection>,
    mut connection_manager: ResMut<ConnectionManager>,
    mut replication_target: Query<(&mut ReplicationTarget, &mut SyncTarget)>,
    mut commands: Commands,
) {
    for event in events.read() {
        let client_id = event.context();
        let lobby_id = lobbies.lobbies[0].lobby_id;
        info!("Inserted player {} unto lobby {}", client_id, lobby_id);
        lobbies.lobbies[0].players.push(*client_id);

        let all_players = lobbies.lobbies[0].players.clone();

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

// Listening for clients who somehow exited the games
fn listener_exit_lobby(
    mut events: EventReader<MessageEvent<ExitLobby>>,
    player_entity_map: Res<PlayerEntityMap>,
    mut online_state: Query<&mut PlayerStateConnection>,
    mut lobbies: ResMut<Lobbies>,
) {
    for event in events.read() {
        let client_id = event.context();

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
