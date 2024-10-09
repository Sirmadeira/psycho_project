//! Basically all the events associated to lobby logic
use crate::server::player::*;
use crate::shared::protocol::lobby_structs::*;
use crate::shared::protocol::player_structs::*;
use bevy::prelude::*;
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

        // Listens to event sent by client
        app.add_systems(Update, listener_search_match);
        app.add_systems(Update, listener_stop_search);
        // Creates a lobby
        app.add_systems(Update, create_lobby);
    }
}

fn replicate_resource(mut commands: Commands) {
    // Replicating resources to clients
    commands.replicate_resource::<Lobbies, Channel1>(NetworkTarget::All);
}

// Responsible for searching for match
fn listener_search_match(
    mut events: EventReader<MessageEvent<SearchMatch>>,
    player_entity_map: Res<PlayerEntityMap>,
    mut online_state: Query<&mut PlayerStateConnection>,
) {
    for event in events.read() {
        let client_id = event.context();

        let player_entity = player_entity_map
            .0
            .get(client_id)
            .expect("To find player in map when searching for his player state");

        let mut on_state = online_state
            .get_mut(*player_entity)
            .expect("For online player to have player state component");

        *on_state = PlayerStateConnection {
            online: true,
            searching: true,
            in_game: false,
        }
    }
}

// Responsible for stop searhcing for match
fn listener_stop_search(
    mut events: EventReader<MessageEvent<StopSearch>>,
    player_entity_map: Res<PlayerEntityMap>,
    mut online_state: Query<&mut PlayerStateConnection>,
) {
    for event in events.read() {
        let client_id = event.context();

        let player_entity = player_entity_map
            .0
            .get(client_id)
            .expect("To find player in map when searching for his player state");

        let mut on_state = online_state
            .get_mut(*player_entity)
            .expect("For online player to have player state component");

        *on_state = PlayerStateConnection {
            online: true,
            searching: false,
            in_game: false,
        }
    }
}

// Creates a lobby if two players are actively searching
pub(crate) fn create_lobby(
    mut lobbies: ResMut<Lobbies>,
    players: Res<PlayerEntityMap>,
    mut query: Query<(&PlayerId, &mut PlayerStateConnection), With<PlayerStateConnection>>,
    mut connection_manager: ResMut<ConnectionManager>,
    mut commands: Commands,
) {
    // Client id searching
    let mut clients_searching = Vec::default();

    let mut online_states = Vec::default();

    // Loop through find total of players searching
    for (client_id, player_state) in query.iter_mut() {
        if player_state.searching == true {
            clients_searching.push(client_id.0);
            online_states.push(player_state);
        }
    }

    // If two are found make it so creates a lobby changes their states
    if clients_searching.len() % 2 == 0 && clients_searching.len() != 0 {
        let mut lobby = Lobby::default();

        info!("Grabbing lobby id");
        let lobby_id = lobbies.lobbies.len();
        lobby.lobby_id = lobby_id;

        info!("Changing player network states to in game");
        for state in online_states.iter_mut() {
            state.in_game = true;
            state.searching = false;
        }
        info!("Sending message to specific clients to start their game and start replicating their player entities");
        for client in clients_searching {
            info!("Defining type of replicatinon");
            let replicate = Replicate {
                sync: SyncTarget {
                    prediction: NetworkTarget::Single(client),
                    interpolation: NetworkTarget::AllExceptSingle(client),
                },
                controlled_by: ControlledBy {
                    target: NetworkTarget::Single(client),
                    ..default()
                },
                ..default()
            };
            if let Some(player) = players.0.get(&client) {
                info!("Replicate the player to all clients");
                commands.entity(*player).insert(replicate);
                lobby.players.push(client);
            } else {
                error!("Couldnt grab player from resource")
            }
            let _ = connection_manager
                .send_message::<Channel1, StartGame>(client, &mut StartGame { lobby_id });
        }
        info!("Creating lobby and replicating to others {}", lobby_id);
        lobbies.lobbies.push(lobby);
    }
}
