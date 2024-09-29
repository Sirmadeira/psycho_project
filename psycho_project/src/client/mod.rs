//! The client plugin.
//! This plugin will act as the entire client meaning every single plugin that relies on the user computer is gonna run here

use bevy::prelude::*;
use lightyear::prelude::client::ClientCommands;
use lightyear::shared::events::components::MessageEvent;

use crate::shared::protocol::lobby_structs::Lobbies;
use crate::shared::protocol::player_structs::PlayerBundleMap;
use crate::{client::ui::UiPlugin, shared::protocol::lobby_structs::StartGame};

mod change_res;
mod form_player;
mod load_assets;
pub mod rtt;
mod ui;

// SElLF MADE IMPORTS
use self::change_res::ChangeResPlugin;
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
        app.add_plugins(ChangeResPlugin);
        app.add_plugins(LoadingAssetsPlugin);
        app.add_plugins(UiPlugin);
        app.add_plugins(CreateCharPlugin);
        app.add_plugins(FormRttsPlugin);

        // Connection systems - Systems that dialogues with server
        app.add_systems(OnEnter(MyAppState::MainMenu), connect_client);

        app.add_systems(Update, start_game);
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum MyAppState {
    #[default]
    // Started loading assets
    LoadingAssets,
    // In main menu for setting player options and such
    MainMenu,
    Lobby,
    Game,
}

// Rc - Only run this system if it has all assets available
pub fn is_loaded(state: Res<State<MyAppState>>) -> bool {
    if *state != MyAppState::LoadingAssets {
        return true;
    } else {
        return false;
    }
}

// First thing we will do is connect the client to server as our server is really important for grabing specific info
pub fn connect_client(mut commands: Commands) {
    info!("Gonna connect to server");
    commands.connect_client();
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
