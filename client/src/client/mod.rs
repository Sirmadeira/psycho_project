//! The client plugin.
//! This plugin will act as the entire client meaning every single plugin that relies on the user computer is gonna run here

use bevy::prelude::*;
use lightyear::shared::events::components::MessageEvent;

use crate::{client::ui::UiPlugin, shared::protocol::lobby_structs::StartGame};

mod create_char;
mod load_assets;
mod ui;

use self::load_assets::LoadingAssetsPlugin;

// Centralization of plugins
pub struct ExampleClientPlugin;

impl Plugin for ExampleClientPlugin {
    fn build(&self, app: &mut App) {
        // Self made plugins
        app.add_plugins(LoadingAssetsPlugin);
        app.add_plugins(UiPlugin);
        app.add_systems(Update, start_game);
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum MyAppState {
    #[default]
    LoadingAssets,
    MainMenu,
    Lobby,
}

// Reads messa
pub fn start_game(mut events: EventReader<MessageEvent<StartGame>>) {
    for event in events.read() {
        let content = event.message();
        info!("Start game {}", content.lobby_id);
    }
}
