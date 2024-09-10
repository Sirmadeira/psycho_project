//! Essential systems to run server

use crate::shared::protocol::lobby_structs::*;
use crate::shared::protocol::player_structs::*;
use bevy::prelude::*;
use lightyear::prelude::server::*;
use lightyear::prelude::*;

// Start the server
pub(crate) fn start_server(mut commands: Commands) {
    // Replicates to all channels
    commands.replicate_resource::<Lobbies, Channel1>(NetworkTarget::All);
    commands.start_server();
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
}

// Handles connections
pub(crate) fn handle_connections(
    mut connections: EventReader<ConnectEvent>,
    mut commands: Commands,
) {
    for connection in connections.read() {
        spawn_player_entity(&mut commands, connection.client_id, false);
    }
}

/// Helper function spawns repicable players
pub(crate) fn spawn_player_entity(
    commands: &mut Commands,
    client_id: ClientId,
    dedicated_server: bool,
) {
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

    let entity = commands.spawn((PlayerBundle::new(client_id, Vec2::ZERO), name, replicate));
    info!("Create entity {:?} for client {:?}", entity.id(), client_id);
}

/// Handle client disconnections: we want to despawn every entity that was controlled by that client.
///
/// Lightyear creates one entity per client, which contains metadata associated with that client.
/// You can find that entity by calling `ConnectionManager::client_entity(client_id)`.
///
/// That client entity contains the `ControlledEntities` component, which is a set of entities that are controlled by that client.
///
/// By default, lightyear automatically despawns all the `ControlledEntities` when the client disconnects;
/// but in this example we will also do it manually to showcase how it can be done.
/// (however we don't actually run the system)
pub(crate) fn handle_disconnections(
    mut disconnections: EventReader<DisconnectEvent>,
    mut lobbies: Option<ResMut<Lobbies>>,
) {
    for disconnection in disconnections.read() {
        // NOTE: games hosted by players will disappear from the lobby list since the host
        //  is not connected anymore
        if let Some(lobbies) = lobbies.as_mut() {
            lobbies.remove_client(disconnection.client_id);
        }
    }
}
