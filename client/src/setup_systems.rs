use bevy::prelude::*;
use lightyear::client::input::native::InputManager;
use lightyear::prelude::client::*;
use lightyear::shared::tick_manager::TickManager;
use shared::protocol::Direction;
use shared::protocol::Inputs;

pub fn start_client(mut commands: Commands) {
    commands.connect_client();
}

/// Add some debugging text to the screen
pub fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(
        TextBundle::from_section(
            "Client",
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

pub(crate) fn buffer_input(
    tick_manager: Res<TickManager>,
    mut input_manager: ResMut<InputManager<Inputs>>,
    keypress: Res<ButtonInput<KeyCode>>,
) {
    let tick = tick_manager.tick();
    let mut input = Inputs::None;
    let mut direction = Direction {
        up: false,
        down: false,
        left: false,
        right: false,
    };
    if keypress.pressed(KeyCode::KeyW) || keypress.pressed(KeyCode::ArrowUp) {
        direction.up = true;
    }
    if keypress.pressed(KeyCode::KeyS) || keypress.pressed(KeyCode::ArrowDown) {
        direction.down = true;
    }
    if keypress.pressed(KeyCode::KeyA) || keypress.pressed(KeyCode::ArrowLeft) {
        direction.left = true;
    }
    if keypress.pressed(KeyCode::KeyD) || keypress.pressed(KeyCode::ArrowRight) {
        direction.right = true;
    }
    if !direction.is_none() {
        input = Inputs::Direction(direction);
    }
    if keypress.pressed(KeyCode::Backspace) {
        input = Inputs::Delete;
    }
    if keypress.pressed(KeyCode::Space) {
        input = Inputs::Spawn;
    }
    input_manager.add_input(input, tick)
}
