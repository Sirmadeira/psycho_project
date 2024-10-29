//! All logic associated to player
use crate::server::save_file;
use crate::shared::protocol::player_structs::*;
use crate::shared::shared_behavior::CharacterPhysicsBundle;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier3d::prelude::Velocity;
use bincode::deserialize_from;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use std::fs::File;
use std::io::BufReader;

/// Plugin for logics that are associated to player creation customization and so on
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Initializing resources
        app.init_resource::<PlayerAmount>();
        app.init_resource::<PlayerEntityMap>();

        // Debug registering
        app.register_type::<PlayerStateConnection>();

        // Replication of resource
        app.add_systems(Startup, replicate_resource);

        // Reads player bundle map and make it readily available when server boots up
        app.add_systems(Startup, read_save_files);

        // Listens to client sent events
        app.add_systems(Update, listener_save_visuals);

        // What happens when you connects to server
        app.add_systems(Update, handle_connections);

        // What happens when you disconnect from server
        app.add_systems(Update, handle_disconnections);
    }
}

/// Current amount of online players
#[derive(Resource, Default)]
pub struct PlayerAmount {
    quantity: u32,
}

/// A server side map, that tells me corresponding player entity according to id
#[derive(Resource, Clone, Default, Reflect)]
#[reflect(Resource, Default)]
pub struct PlayerEntityMap(pub HashMap<ClientId, Entity>);

/// State of connection of our player
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
pub struct PlayerStateConnection {
    pub online: bool,
    // If in game or not
    pub in_game: bool,
}

fn replicate_resource(mut commands: Commands) {
    commands.replicate_resource::<PlayerBundleMap, Channel1>(NetworkTarget::All);
}

/// Reads current save files and fill up the resource playerbundlemap each basically gives me all player info
fn read_save_files(mut commands: Commands) {
    let f = BufReader::new(
        File::open("./psycho_project/src/server/save_files/player_info.bar").unwrap(),
    );
    let player_bundle_map: PlayerBundleMap = deserialize_from(f).unwrap();
    info!("Read from save file: {:?}", player_bundle_map);

    commands.insert_resource(player_bundle_map);
}

/// Responsible for saving player info
fn listener_save_visuals(
    mut events: EventReader<MessageEvent<SaveVisual>>,
    mut player_map: ResMut<PlayerBundleMap>,
    player_entity_map: Res<PlayerEntityMap>,
    mut server_player_visuals: Query<&mut PlayerVisuals>,
) {
    for event in events.read() {
        let client_id = event.context();

        info!("Grabbing player visuals and body part to change from client");
        let client_player_visuals = event.message().0.clone();

        info!("Saving player info {}", client_id);

        if let Some(player_bundle) = player_map.0.get_mut(client_id) {
            info!("Found it is bundle and changing visual  for what client said in resource player bundle map");
            player_bundle.visuals = client_player_visuals.clone();
            if let Some(server_player) = player_entity_map.0.get(client_id) {
                info!("Grabbing server player and also adjusting his component");
                let mut server_visuals = server_player_visuals
                    .get_mut(*server_player)
                    .expect("Server player to have visuals");
                *server_visuals = client_player_visuals.clone();
            } else {
                error!("Couldnt find server player something went wrong")
            }

            info!("Saving this bundle {:?}", player_bundle);
            save_file(player_map.clone());
        } else {
            error!("Something went wrong in grabing this id info in server");
        }
    }
}

/// Helper function spawns the player that is gonna be replciated
pub(crate) fn spawn_server_player(
    client_id: ClientId,
    commands: &mut Commands,
    player_bundle: Option<PlayerBundle>,
    player_entity_map: &mut ResMut<PlayerEntityMap>,
) -> PlayerBundle {
    let name = Name::new(format!("Player {:?}", client_id));

    info!("Setting their status to online");
    let online_state = PlayerStateConnection {
        online: true,
        in_game: false,
    };

    if let Some(old_player_bun) = player_bundle {
        info!("Inserting into entity map resource");
        let id = commands
            .spawn(old_player_bun.clone())
            .insert(online_state)
            .insert(name)
            .insert(CharacterPhysicsBundle::default())
            .insert(Velocity::zero())
            .id();
        player_entity_map.0.insert(client_id, id);
        return old_player_bun;
    } else {
        info!("Inserting new player into entity map resource");
        // Setting default visuals
        let player_visual = PlayerVisuals::default();
        let player_position = PlayerPosition::default();
        let new_player_bundle = PlayerBundle::new(client_id, player_visual, player_position);
        let id = commands
            .spawn(new_player_bundle.clone())
            .insert(online_state)
            .insert(name)
            .insert(CharacterPhysicsBundle::default())
            .insert(Velocity::zero())
            .id();

        player_entity_map.0.insert(client_id, id);
        return new_player_bundle;
    }
}

/// Spawns a server player everytime someone connects
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
            spawn_server_player(
                connection.client_id,
                &mut commands,
                Some(old_player_bundle.clone()),
                &mut player_entity_map,
            );
        } else {
            info!("New player make him learn! And insert him into resource");
            let new_bundle = spawn_server_player(
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

/// Spawns a player everytime someone disconnects
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
