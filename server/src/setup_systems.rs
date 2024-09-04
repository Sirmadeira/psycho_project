use bevy::prelude::*;
use lightyear::prelude::server::ServerCommands;

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
