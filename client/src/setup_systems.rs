use bevy::prelude::*;
use lightyear::prelude::client::*;

pub fn start_client(mut commands: Commands) {
    commands.connect_client();
}

/// Add some debugging text to the screen
pub fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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

pub(crate) fn handle_disconnection(mut events: EventReader<DisconnectEvent>) {
    for event in events.read() {
        let reason = &event.reason;
        error!("Disconnected from server: {:?}", reason);
    }
}
