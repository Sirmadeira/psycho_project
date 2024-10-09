//! The client plugin.
//! This plugin will act as the entire client meaning every single plugin that relies on the user computer is gonna run here

use crate::client::ui::UiPlugin;
use crate::shared::protocol::lobby_structs::Lobbies;
use crate::shared::protocol::player_structs::PlayerBundleMap;
use bevy::prelude::*;

mod change_res;
pub mod essentials;
mod form_player;
mod load_assets;
pub mod rtt;
mod ui;

// SElLF MADE IMPORTS
use self::change_res::ChangeResPlugin;
use self::essentials::SystemsPlugin;
use self::form_player::CreateCharPlugin;
use self::load_assets::LoadingAssetsPlugin;
use self::rtt::FormRttsPlugin;

// OTHER PLUGINS
use bevy_panorbit_camera::PanOrbitCameraPlugin;

// Centralization of plugins
pub struct ExampleClientPlugin;

impl Plugin for ExampleClientPlugin {
    fn build(&self, app: &mut App) {
        // Inserting resources that must exist first
        app.insert_resource(Lobbies::default());
        app.insert_resource(PlayerBundleMap::default());
        // Initializing states that must exist
        app.init_state::<MyAppState>();

        // Debugging
        app.register_type::<Lobbies>();
        app.register_type::<PlayerBundleMap>();

        //Imported plugins
        app.add_plugins(PanOrbitCameraPlugin);

        // Self made plugins
        app.add_plugins(SystemsPlugin);
        app.add_plugins(ChangeResPlugin);
        app.add_plugins(LoadingAssetsPlugin);
        app.add_plugins(UiPlugin);
        app.add_plugins(CreateCharPlugin);
        app.add_plugins(FormRttsPlugin);

        // Connection systems - Systems that dialogues with server
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum MyAppState {
    #[default]
    // Started loading assets
    LoadingAssets,
    // In main menu for setting player options and such
    MainMenu,
    // Lobby screen
    Pause,
    // Inventory sub-screens
    Inventory,
    // Ingame
    Game,
}
