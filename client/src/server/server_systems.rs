//! Essential systems to run server

use crate::shared::protocol::lobby_structs::*;
use crate::shared::protocol::player_structs::*;
use bevy::prelude::*;
use bevy::utils::info;
use bevy::utils::HashMap;
use lightyear::prelude::server::*;
use lightyear::prelude::*;

use rand::seq::IteratorRandom;

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
    client_ids: HashMap<ClientId, bool>,
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
        spawn_player_entity(&mut commands, connection.client_id, false);
        current_players.quantity += 1;
        info!("Current players online is {}", current_players.quantity);
        current_players
            .client_ids
            .insert(connection.client_id, true);
    }
}

// Creates a lobby after two players are connected automatically
pub fn create_lobby(
    mut lobbies: ResMut<Lobbies>,
    current_players: ResMut<PlayerAmount>,
    mut connection_manager: ResMut<ConnectionManager>,
) {
    let mut lobby = Lobby::default();

    let rng = &mut rand::thread_rng();

    let sample = current_players.client_ids.keys();
    if current_players.quantity % 2 == 0 {
        let vec = sample.choose_multiple(rng, 2);
        lobby.players.extend(vec.clone());
        lobby.in_game = true;
        info("Creating lobby");
        lobbies.lobbies.push(lobby);

        let lobby_id = lobbies.lobbies.len() + 1;

        for client_id in vec.iter() {
            let _ = connection_manager
                .send_message::<Channel1, StartGame>(**client_id, &mut StartGame { lobby_id });
            info!("Starting game");
        }
    }
}

/// Helper function spawns repicable players
pub(crate) fn spawn_player_entity(
    commands: &mut Commands,
    client_id: ClientId,
    dedicated_server: bool,
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

    let entity = commands.spawn((PlayerBundle::new(client_id), name, replicate));
    info!("Create entity {:?} for client {:?}", entity.id(), client_id);
    return entity.id();
}
