//! Essential systems to run when server boot ups or connections occur to him
use crate::server::save_file;
use crate::shared::protocol::lobby_structs::*;
use crate::shared::protocol::player_structs::*;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
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

/// Helper function spawns repicable players - TURN THIS TO LOOP
pub(crate) fn spawn_player_entity(
    client_id: ClientId,
    dedicated_server: bool,
    commands: &mut Commands,
    player_bundle: Option<PlayerBundle>,
) -> Option<PlayerBundle> {
    // Replicating component important to define who sees the player or not
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

    info!("Settin their status to searching for matchmaking");
    let player_state = PlayStateConnection {
        searching: true,
        in_game: false,
    };

    if let Some(player_bun) = player_bundle {
        commands
            .spawn(player_bun)
            .insert(player_state)
            .insert(name)
            .insert(replicate);
        info!("Replicating veteran player");
        return None;
    } else {
        // Setting default visuals
        let player_visual = PlayerVisuals::default();
        let new_player_bundle = PlayerBundle::new(client_id, player_visual);
        commands
            .spawn(new_player_bundle.clone())
            .insert(player_state)
            .insert(name)
            .insert(replicate);
        info!("Replicating new player");
        return Some(new_player_bundle);
    }
}

// Handles connections
pub(crate) fn handle_connections(
    mut current_players: ResMut<PlayerAmount>,
    mut connections: EventReader<ConnectEvent>,
    mut connection_manager: ResMut<ConnectionManager>,
    mut player_map: ResMut<PlayerBundleMap>,
    mut commands: Commands,
) {
    for connection in connections.read() {
        info!("Checking if new client or if already exists");
        if let Some(old_player_bundle) = player_map.0.get(&connection.client_id) {
            info!(
                "This player {:?} already connected once spawn it is entity according to it is settings",old_player_bundle.id
            );
            spawn_player_entity(
                connection.client_id,
                false,
                &mut commands,
                Some(old_player_bundle.clone()),
            );
            info!("Sending message to client for RTT his given character");
            let client_visuals = old_player_bundle.visuals.clone();

            let _ = connection_manager.send_message::<Channel1, PlayerLoadout>(
                connection.client_id,
                &mut PlayerLoadout(client_visuals),
            );
        } else {
            info!("New player make him learn! And insert him into resource");
            let new_bundle =
                spawn_player_entity(connection.client_id, false, &mut commands, None).unwrap();

            player_map
                .0
                .insert(connection.client_id, new_bundle.clone());

            info!("New player visuals being sent for RTT obviusly default");
            let client_visuals = new_bundle.visuals;

            let _ = connection_manager.send_message::<Channel1, PlayerLoadout>(
                connection.client_id,
                &mut PlayerLoadout(client_visuals),
            );

            info!("Saving player info in file for first time");
            save_file(player_map.clone());
        }

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
