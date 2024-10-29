use crate::client::ui::UiPlugin;
use crate::shared::protocol::lobby_structs::Lobbies;
use crate::shared::protocol::player_structs::PlayerBundleMap;
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

mod change_res;
mod essentials;
mod load_assets;
mod physical_world;
mod player;
pub mod rtt;
mod start_game;
mod ui;
mod voxel_gen;

// SElLF MADE IMPORTS
use self::change_res::ChangeResPlugin;
use self::essentials::SystemsPlugin;
use self::load_assets::LoadingAssetsPlugin;
use self::physical_world::PhysicalWorldPlugin;
use self::player::CreateCharPlugin;
use self::rtt::FormRttsPlugin;
use self::start_game::InGamePlugin;
use self::voxel_gen::VoxelGenPlugin;

/// Important plugin centralizes most of our client related logic
pub struct ExampleClientPlugin;

impl Plugin for ExampleClientPlugin {
    fn build(&self, app: &mut App) {
        // Inserting resources that must exist first
        app.insert_resource(Lobbies::default());
        app.insert_resource(PlayerBundleMap::default());
        // Initializing states that must exist
        app.init_state::<MyAppState>();

        //Imported plugins - Made by others
        app.add_plugins(PanOrbitCameraPlugin);

        // Self made plugins
        app.add_plugins(SystemsPlugin);
        app.add_plugins(ChangeResPlugin);
        app.add_plugins(LoadingAssetsPlugin);
        app.add_plugins(UiPlugin);
        app.add_plugins(CreateCharPlugin);
        app.add_plugins(FormRttsPlugin);
        app.add_plugins(InGamePlugin);
        app.add_plugins(VoxelGenPlugin);
        app.add_plugins(PhysicalWorldPlugin);
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
