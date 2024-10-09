//! Essential systems to run when server boot ups or connections occur to him
use bevy::prelude::*;
use lightyear::prelude::server::*;

pub struct EssentialsPlugin;

impl Plugin for EssentialsPlugin {
    fn build(&self, app: &mut App) {
        // Init server in head mode
        app.add_systems(Startup, (start_server, init));
    }
}

// Start the server
pub(crate) fn start_server(mut commands: Commands) {
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

    // Camera to avoid boring warning
    commands.spawn(Camera2dBundle::default());
}
