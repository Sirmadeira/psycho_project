use bevy::prelude::*;
use lightyear::prelude::server::ServerCommands;
use lightyear::prelude::server::*;
use lightyear::shared::replication::network_target::NetworkTarget;
use shared::protocol::PlayerBundle;

pub fn start_server(mut commands: Commands) {
    commands.start_server();
}

/// Add some debugging text to the screen
pub fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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

// When player connects to server well creates a player
pub(crate) fn handle_connections(
    mut connections: EventReader<ConnectEvent>,
    mut commands: Commands,
) {
    for connection in connections.read() {
        let client_id = connection.client_id;
        info_once!("This is the current client connected");
        let replicate = Replicate {
            sync: SyncTarget {
                prediction: NetworkTarget::Single(client_id),
                interpolation: NetworkTarget::AllExceptSingle(client_id),
            },
            controlled_by: ControlledBy {
                target: NetworkTarget::Single(client_id),
                ..default()
            },
            ..default()
        };
        let entity = commands.spawn((PlayerBundle::new(client_id, Vec2::ZERO), replicate));
        info!("Create entity {:?} for client {:?}", entity.id(), client_id);
    }
}
