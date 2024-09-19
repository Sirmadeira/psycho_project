//! Essential systems to run server

use crate::shared::protocol::lobby_structs::*;
use crate::shared::protocol::player_structs::*;
use bevy::prelude::*;
use lightyear::prelude::server::*;
use lightyear::prelude::*;

// Start the server
pub(crate) fn start_server(mut commands: Commands) {
    commands.start_server();
    // Replicating resources to clients
    commands.replicate_resource::<Lobbies, Channel1>(NetworkTarget::All);
    commands.replicate_resource::<PlayerBundleMap, Channel1>(NetworkTarget::All);
}

// Gives me the current player amount
#[derive(Resource, Default)]
pub struct PlayerAmount {
    quantity: u32,
}

// Tells me the player/client state of connection
#[derive(Component, Default, Clone)]
pub struct PlayStateConnection {
    // If is searching or not
    searching: bool,
    // If in game or not
    in_game: bool,
}

/// Add some debugging text to the screen
pub(crate) fn init(mut commands: Commands) {
    commands.spawn(
        TextBundle::from_section(
            "Server",
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            align_self: AlignSelf::End,
            ..default()
        }),
    );

    // Camera to avoid boring warning
    commands.spawn(Camera2dBundle::default());
}

// Handles connections
pub(crate) fn handle_connections(
    mut current_players: ResMut<PlayerAmount>,
    mut connections: EventReader<ConnectEvent>,
    mut commands: Commands,
    mut player_map: ResMut<PlayerBundleMap>,
) {
    for connection in connections.read() {
        info!("Settin their status to searching for matchmaking");
        let player_state = PlayStateConnection {
            searching: true,
            in_game: false,
        };

        let (client_id, player_bundle) = spawn_player_entity(
            &mut commands,
            connection.client_id,
            false,
            player_state.clone(),
        );

        info!("Inserting into save resource configs from that client");
        player_map.0.insert(client_id, player_bundle);

        current_players.quantity += 1;
        info!("Current players online is {}", current_players.quantity);
    }
}

pub(crate) fn handle_disconnections(
    mut disconnections: EventReader<DisconnectEvent>,
    mut current_players: ResMut<PlayerAmount>,
) {
    for disconnection in disconnections.read() {
        let client_id = disconnection.client_id;
        info!("Client disconnected {}", client_id);
        current_players.quantity -= 1;
    }
}

// Creates a lobby if two players are actively searching
pub(crate) fn create_lobby(
    mut lobbies: ResMut<Lobbies>,
    mut query: Query<(&PlayerId, &mut PlayStateConnection), With<PlayStateConnection>>,
    mut connection_manager: ResMut<ConnectionManager>,
) {
    // Client id searching
    let mut clients_searching = Vec::default();

    let mut player_states = Vec::default();

    // Loop through find total of players searching
    for (client_id, player_state) in query.iter_mut() {
        if player_state.searching == true {
            clients_searching.push(client_id.0);
            player_states.push(player_state);
        }
    }

    // If two are found make it so creates a lobby changes their states
    if clients_searching.len() % 2 == 0 && clients_searching.len() != 0 {
        let mut lobby = Lobby::default();
        info!("Grabbing lobby id");
        let lobby_id = lobbies.lobbies.len();
        lobby.lobby_id = lobby_id;
        info!("Changing player network states to in game");
        for state in player_states.iter_mut() {
            state.in_game = true;
            state.searching = false;
        }
        info!("Sending message to specific clients to start their game");
        for clients in clients_searching {
            lobby.players.push(clients);
            let _ = connection_manager
                .send_message::<Channel1, StartGame>(clients, &mut StartGame { lobby_id });
        }
        info!("Creating lobby and replicating to others {}", lobby_id);
        lobbies.lobbies.push(lobby);
    }
}

/// Helper function spawns repicable players - TURN THIS TO LOOP
pub(crate) fn spawn_player_entity(
    commands: &mut Commands,
    client_id: ClientId,
    dedicated_server: bool,
    player_state: PlayStateConnection,
) -> (ClientId, PlayerBundle) {
    let replicate = Replicate {
        sync: SyncTarget {
            prediction: NetworkTarget::Single(client_id),
            interpolation: NetworkTarget::AllExceptSingle(client_id),
        },
        controlled_by: ControlledBy {
            target: NetworkTarget::Single(client_id),
            ..default()
        },
        relevance_mode: if dedicated_server {
            NetworkRelevanceMode::InterestManagement
        } else {
            NetworkRelevanceMode::All
        },
        ..default()
    };

    let name = Name::new(format!("Player {:?}", client_id));

    let player_bundle = PlayerBundle::new(client_id);

    let entity = commands.spawn((player_bundle.clone(), player_state, name, replicate));
    info!("Create entity {:?} for client {:?}", entity.id(), client_id);

    return (client_id, player_bundle);
}
