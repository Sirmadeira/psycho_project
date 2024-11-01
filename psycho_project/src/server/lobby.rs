//! Basically all the events associated to lobby logic
use crate::server::player::*;
use crate::shared::physics::*;
use crate::shared::protocol::lobby_structs::*;
use crate::shared::protocol::player_structs::*;
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

/// Creates the major lobby for players
fn create_lobby(mut lobbies: ResMut<Lobbies>) {
    let mut lobby = Lobby::default();

    info!("Grabbing lobby id");
    let lobby_id = lobbies.lobbies.len();
    lobby.lobby_id = lobby_id;

    info!("Creating lobby and replicating to others {}", lobby_id);
    lobbies.lobbies.push(lobby);
}

/// Listening for clients that clicked the button start game
fn listener_join_lobby(
    mut online_state: Query<&mut PlayerStateConnection>,
    mut events: EventReader<MessageEvent<EnterLobby>>,
    player_entity_map: Res<PlayerEntityMap>,
    mut lobbies: ResMut<Lobbies>,
    mut connection_manager: ResMut<ConnectionManager>,
    mut commands: Commands,
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

        info!("Adjusting player state to in_game");
        *on_state = PlayerStateConnection {
            online: true,
            in_game: true,
        };
        let lobby_id = lobbies.lobbies[0].lobby_id;
        info!("Inserted player {} unto lobby {}", client_id, lobby_id);
        lobbies.lobbies[0].players.push(*client_id);

        info!("Defining type of replicatinon for that player important to know he is from replication_group 1");
        let replicate = Replicate {
            sync: SyncTarget {
                prediction: NetworkTarget::All,
                ..default()
            },
            controlled_by: ControlledBy {
                target: NetworkTarget::Single(*client_id),
                ..default()
            },
            group: REPLICATION_GROUP,
            ..default()
        };

        info!("Replicate the player to all client_ids");
        commands
            .entity(*player_entity)
            .insert(replicate)
            .insert(CharacterPhysicsBundle::default())
            .insert(ActionState::<CharacterAction>::default());

        info!("Starting game for client {}", client_id);
        let _ = connection_manager
            .send_message::<Channel1, StartGame>(*client_id, &mut StartGame { lobby_id });
    }
}

// Listening for clients who somehow exited the games
fn listener_exit_lobby(
    mut events: EventReader<MessageEvent<ExitLobby>>,
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
            in_game: false,
        }
    }
}
