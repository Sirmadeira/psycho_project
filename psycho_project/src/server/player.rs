//! All logic associated to player
use crate::server::save_file;
use crate::shared::protocol::lobby_structs::*;
use crate::shared::protocol::player_structs::*;
use crate::shared::protocol::weapon_structs::Weapon;
use crate::shared::protocol::CommonChannel;
use crate::shared::shared_gun::process_collisions;
use crate::shared::shared_gun::shared_spawn_bullet;
use crate::shared::shared_gun::BulletHitEvent;
use crate::shared::shared_physics::*;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bincode::deserialize_from;
use leafwing_input_manager::prelude::ActionState;
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

        // this system will replicate the inputs of a client to other clients
        // so that a client can predict other clients
        app.add_systems(PreUpdate, replicate_inputs.after(MainSet::EmitEvents));

        // Reads player bundle map and make it readily available when server boots up
        app.add_systems(Startup, read_save_files);

        // Listens to client sent events
        app.add_systems(Update, listener_save_visuals);

        // What happens when you connects to server
        app.add_systems(Update, handle_connections);

        // What happens when you disconnect from server
        app.add_systems(Update, handle_disconnections);

        app.add_systems(FixedUpdate, insert_physics_server_player);

        // It is essential that input based systems occur in fixedupdate
        app.add_systems(
            FixedUpdate,
            handle_character_actions.in_set(InputPhysicsSet::Input),
        );

        app.add_systems(
            FixedUpdate,
            shared_spawn_bullet.in_set(InputPhysicsSet::Input),
        );
        app.add_systems(
            FixedUpdate,
            handle_bullet_hit
                .run_if(on_event::<BulletHitEvent>())
                .after(process_collisions),
        );
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
    commands.replicate_resource::<PlayerBundleMap, CommonChannel>(NetworkTarget::All);
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

/// Helper function spawns the player that is gonna be replicated
fn spawn_server_player(
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

    let replicate = Replicate {
        target: ReplicationTarget {
            target: NetworkTarget::None,
        },
        controlled_by: ControlledBy {
            target: NetworkTarget::Single(client_id),
            ..default()
        },
        sync: SyncTarget {
            prediction: NetworkTarget::None,
            ..default()
        },
        group: REPLICATION_GROUP,
        ..default()
    };

    if let Some(old_player_bun) = player_bundle {
        info!("Inserting into entity map resource");
        let id = commands
            .spawn(old_player_bun.clone())
            .insert(online_state)
            .insert(name)
            .insert(replicate)
            .insert(CharacterAction::default_input_map())
            .insert(Weapon::default())
            .insert(Position(Vec3::new(0.0, 2.0, 0.0)))
            .insert(PlayerHealth::default())
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
            .insert(replicate)
            .insert(CharacterAction::default_input_map())
            .insert(Weapon::default())
            .insert(Position(Vec3::new(0.0, 2.0, 0.0)))
            .insert(PlayerHealth::default())
            .id();

        player_entity_map.0.insert(client_id, id);
        return new_player_bundle;
    }
}

/// Spawns a server player everytime someone connects
fn handle_connections(
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
fn handle_disconnections(
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
            // Right now lightyear does that for us
        } else {
            error!("Player entity not found for client ID: {}", client_id);
        }
    }
}

fn replicate_inputs(
    mut connection: ResMut<ConnectionManager>,
    mut input_events: ResMut<Events<MessageEvent<InputMessage<CharacterAction>>>>,
    lobby_position_map: Res<LobbyPositionMap>,
) {
    for mut event in input_events.drain() {
        let client_id = *event.context();
        if let Some(client_info) = lobby_position_map.0.get(&client_id) {
            let client_info = &client_info.lobby_without_me;
            // Optional here you can validate input
            connection
                .send_message_to_target::<InputChannel, _>(
                    &mut event.message,
                    NetworkTarget::Only(client_info.to_vec()),
                )
                .unwrap()
        }
    }
}
fn handle_character_actions(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    mut query: Query<(&ActionState<CharacterAction>, CharacterQuery)>,
) {
    for (action_state, mut character) in &mut query {
        apply_character_action(&time, &spatial_query, action_state, &mut character);
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

/// Responsible for encapsulating the bullet hit event and changing player health when occurs
fn handle_bullet_hit(
    mut bullet_hit_event: EventReader<BulletHitEvent>,
    mut player_health: Query<&mut PlayerHealth>,
    entity_map: Res<PlayerEntityMap>,
) {
    for bullet_hit in bullet_hit_event.read() {
        if let Some(_) = entity_map.0.get(&bullet_hit.bullet_owner) {
            if let Some(victim_id) = bullet_hit.victim_client_id {
                if let Some(victim) = entity_map.0.get(&victim_id) {
                    if let Ok(mut player_health) = player_health.get_mut(*victim) {
                        info!(
                            "Shooter id {} just shoot {} diminishing his health",
                            bullet_hit.bullet_owner, victim_id
                        );
                        player_health.0 -= 2;
                    }
                } else {
                    warn!("Couldnt grab victim entity in entity map somethin went terribly wrong")
                }
            } else {
                //TODO HANDLE MISSING - PERHAPS TELL HIM HE IS SHIT?
            }
        } else {
            warn!("Couldnt grab bullet owner something is terribly wrong")
        }
    }
}
