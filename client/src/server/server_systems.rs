//! Essential systems to run server

use crate::shared::protocol::lobby_structs::*;
use crate::shared::protocol::player_structs::*;
use bevy::prelude::*;
use bevy::utils::info;
use bevy::utils::HashMap;
use lightyear::prelude::server::*;
use lightyear::prelude::*;

// Start the server
pub(crate) fn start_server(mut commands: Commands) {
    // Replicates to all channels

    commands.replicate_resource::<Lobbies, Channel1>(NetworkTarget::All);
    commands.start_server();
}

// Gives me the current player amount
#[derive(Resource, Default)]
pub struct PlayerAmount {
    quantity: u32,
}

#[derive(Component, Default, Clone)]
pub struct PlayerState {
    // If in lobby or not
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
) {
    for connection in connections.read() {
        info!("Settin their status to searching for matchmaking");
        let player_state = PlayerState {
            searching: true,
            in_game: false,
        };

        spawn_player_entity(
            &mut commands,
            connection.client_id,
            false,
            player_state.clone(),
        );
        current_players.quantity += 1;
        info!("Current players online is {}", current_players.quantity);
    }
}

// Creates a lobby if two players are actively searching
pub fn create_lobby(
    mut lobbies: ResMut<Lobbies>,
    mut query: Query<(&PlayerId, &mut PlayerState), With<PlayerState>>,
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

    // If two are found make it so creates a lobby and changer their state
    if clients_searching.len() % 2 == 0 && clients_searching.len() != 0 {
        info!("Creating lobby");
        let mut lobby = Lobby::default();
        let lobby_id = lobbies.lobbies.len();
        lobby.lobby_id = lobby_id;
        lobbies.lobbies.push(lobby);
        for state in player_states.iter_mut() {
            state.in_game = true;
            state.searching = false;
        }
    }
}

/// Helper function spawns repicable players
pub(crate) fn spawn_player_entity(
    commands: &mut Commands,
    client_id: ClientId,
    dedicated_server: bool,
    player_state: PlayerState,
) -> Entity {
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

    let entity = commands.spawn((PlayerBundle::new(client_id), name, replicate, player_state));
    info!("Create entity {:?} for client {:?}", entity.id(), client_id);
    return entity.id();
}
