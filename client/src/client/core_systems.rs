use crate::shared::protocol::lobby_structs::Lobbies;
use bevy::prelude::*;
use common::settings::{get_client_net_config, Settings};
use lightyear::connection::client::ClientConnection;
use lightyear::prelude::client::*;
use lightyear::prelude::server::ServerCommands;
use lightyear::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, on_disconnect);
        app.add_systems(PreUpdate, handle_connection.after(MainSet::Receive));
    }
}

/// Component to identify the text displaying the client id
#[derive(Component)]
pub struct ClientIdText;

/// Listen for events to know when the client is connected, and spawn a text entity
/// to display the client id
pub(crate) fn handle_connection(
    mut commands: Commands,
    mut connection_event: EventReader<ConnectEvent>,
) {
    for event in connection_event.read() {
        let client_id = event.client_id();
        commands.spawn((
            TextBundle::from_section(
                format!("Client {}", client_id),
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            ClientIdText,
        ));
    }
}

/// Remove all entities when the client disconnect.
/// Reset the ClientConfig to connect to the dedicated server on the next connection attempt.
fn on_disconnect(
    mut commands: Commands,
    entities: Query<Entity, (Without<Window>, Without<Camera3d>)>,
    mut config: ResMut<ClientConfig>,
    settings: Res<Settings>,
    connection: Res<ClientConnection>,
) {
    let existing_client_id = connection.id();

    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<Lobbies>();

    // stop the server if it was started (if the player was host)
    commands.stop_server();

    // update the client config to connect to the lobby server
    config.net = get_client_net_config(settings.as_ref(), existing_client_id.to_bits());
}
