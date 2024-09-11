//! The client plugin.
//! This plugin will act as the entire client meaning every single plugin that relies on the user computer is gonna run here

use bevy::prelude::*;

use crate::client::ui::UiPlugin;

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
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum MyAppState {
    #[default]
    LoadingAssets,
    MainMenu,
    Lobby,
}
