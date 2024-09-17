//! The client plugin.
//! This plugin will act as the entire client meaning every single plugin that relies on the user computer is gonna run here

use bevy::prelude::*;
use lightyear::shared::events::components::MessageEvent;

use crate::shared::protocol::lobby_structs::Lobbies;
use crate::{client::ui::UiPlugin, shared::protocol::lobby_structs::StartGame};

mod create_char;
mod load_assets;
mod ui;

use self::load_assets::LoadingAssetsPlugin;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

// Centralization of plugins
pub struct ExampleClientPlugin;

impl Plugin for ExampleClientPlugin {
    fn build(&self, app: &mut App) {
        // Inserting resources that must exist first
        app.insert_resource(Lobbies::default());
        // Initializing states that must exist
        app.init_state::<MyAppState>();

        // Debugging
        app.register_type::<Lobbies>();

        //Imported plugins
        app.add_plugins(PanOrbitCameraPlugin);

        // Self made plugins
        app.add_plugins(LoadingAssetsPlugin);
        app.add_plugins(UiPlugin);
        // app.add_plugins(CreateCharPlugin);

        // Listening systems - Systems that hear messages from server
        app.add_systems(Update, start_game);
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum MyAppState {
    #[default]
    LoadingAssets,
    MainMenu,
    Lobby,
    Game,
}

// Starts the game the message filters out the specific clients
pub fn start_game(
    mut events: EventReader<MessageEvent<StartGame>>,
    mut next_state: ResMut<NextState<MyAppState>>,
) {
    for event in events.read() {
        let content = event.message();
        info!("Start game for lobby {}", content.lobby_id);
        next_state.set(MyAppState::Game);
    }
}
