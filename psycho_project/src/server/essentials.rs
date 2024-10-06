//! Essential systems to run when server boot ups or connections occur to him
use crate::server::save_file;
use crate::shared::protocol::lobby_structs::*;
use crate::shared::protocol::player_structs::*;
use bevy::prelude::*;
use bevy::utils::HashMap;
use lightyear::prelude::server::*;
use lightyear::prelude::*;

pub struct EssentialsPlugin;

impl Plugin for EssentialsPlugin {
    fn build(&self, app: &mut App) {
        // Initializing resources
        app.init_resource::<Lobbies>();
        app.init_resource::<PlayerAmount>();
        app.init_resource::<PlayerEntityMap>();

        // Debug registering
        app.register_type::<Lobbies>();
        app.register_type::<PlayerBundleMap>();
        app.register_type::<PlayerStateConnection>();
        app.register_type::<PlayerVisuals>();

        // app.add_systems(Startup, create_save_files);
        // Init server in head mode
        app.add_systems(Startup, (start_server, init));

        // What happens when you connects to server
        app.add_systems(Update, handle_connections);

        // What happens when you disconnect from server
        app.add_systems(Update, handle_disconnections);

        // Creates a lobby
        app.add_systems(Update, create_lobby);
    }
}

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

// A server side map, that tells me corresponding player entity according to id
#[derive(Resource, Clone, Default, Reflect)]
#[reflect(Resource, Default)]
pub struct PlayerEntityMap(pub HashMap<ClientId, Entity>);

// Visuals of our character
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
pub struct PlayerStateConnection {
    pub online: bool,
    // If is searching or not
    pub searching: bool,
    // If in game or not
    pub in_game: bool,
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

/// Helper function spawns repicable players
pub(crate) fn spawn_player_entity(
    client_id: ClientId,
    commands: &mut Commands,
    player_bundle: Option<PlayerBundle>,
    player_entity_map: &mut ResMut<PlayerEntityMap>,
) -> PlayerBundle {
    let name = Name::new(format!("Player {:?}", client_id));

    info!("Setting their status to online");
    let online_state = PlayerStateConnection {
        online: true,
        searching: false,
        in_game: false,
    };

    if let Some(old_player_bun) = player_bundle {
        info!("Inserting into server map resource");
        let id = commands
            .spawn(old_player_bun.clone())
            .insert(online_state)
            .insert(name)
            .id();
        player_entity_map.0.insert(client_id, id);
        return old_player_bun;
    } else {
        info!("Inserting new player into server map");
        // Setting default visuals
        let player_visual = PlayerVisuals::default();
        let new_player_bundle = PlayerBundle::new(client_id, player_visual);
        let id = commands
            .spawn(new_player_bundle.clone())
            .insert(online_state)
            .insert(name)
            .id();

        player_entity_map.0.insert(client_id, id);
        return new_player_bundle;
    }
}

// Handles connections
pub(crate) fn handle_connections(
    mut current_players: ResMut<PlayerAmount>,
    mut connections: EventReader<ConnectEvent>,
    mut player_map: ResMut<PlayerBundleMap>,
    mut player_entity_map: ResMut<PlayerEntityMap>,
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
                &mut commands,
                Some(old_player_bundle.clone()),
                &mut player_entity_map,
            );
        } else {
            info!("New player make him learn! And insert him into resource");
            let new_bundle = spawn_player_entity(
                connection.client_id,
                &mut commands,
                None,
                &mut player_entity_map,
            );

            player_map
                .0
                .insert(connection.client_id, new_bundle.clone());

            info!("Saving player info in file for first time doing this because he wont renember you if you acess twice");
            save_file(player_map.clone());
        }

        current_players.quantity += 1;
        info!("Current players online is {}", current_players.quantity);
    }
}

pub(crate) fn handle_disconnections(
    mut disconnections: EventReader<DisconnectEvent>,
    mut current_players: ResMut<PlayerAmount>,
    mut player_entity_map: ResMut<PlayerEntityMap>,
) {
    for disconnection in disconnections.read() {
        let client_id = disconnection.client_id;
        info!("Client disconnected {}", client_id);

        // Decrease player count
        current_players.quantity -= 1;
        info!("Stop replicating this shitfuck entity on the server");

        // Find and despawn the player's entity
        if let Some(disconnecting_player) = player_entity_map.0.remove(&client_id) {
            info!("This player disconnected {}", disconnecting_player);
        } else {
            error!("Player entity not found for client ID: {}", client_id);
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
