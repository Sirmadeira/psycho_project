//! Essential systems utilized to connect client and so on

use crate::client::MyAppState;
use crate::shared::protocol::lobby_structs::StartGame;
use bevy::prelude::*;
use lightyear::client::events::ConnectEvent;
use lightyear::prelude::client::ClientCommands;
use lightyear::prelude::*;
use lightyear::shared::events::components::MessageEvent;
pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EasyClient>();
        app.add_systems(Startup, connect_client);
        app.add_systems(Update, form_client_id);
        app.add_systems(Update, listener_start_game);
    }
}

// Stores you current client id
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct EasyClient(pub ClientId);

// First thing we will do is connect the client to server as our server is really important for grabing specific info
pub fn connect_client(mut commands: Commands) {
    info!("Gonna connect to server");
    commands.connect_client();
}

// When we have a connect event grab the client id
fn form_client_id(mut connection_event: EventReader<ConnectEvent>, mut commands: Commands) {
    for event in connection_event.read() {
        info!("Forming an easy resource to easily acess things via client id");
        let client_id = event.client_id();
        commands.insert_resource(EasyClient(client_id));
    }
}

// Starts the game the message filters out the specific clients
pub fn listener_start_game(
    mut events: EventReader<MessageEvent<StartGame>>,
    mut next_state: ResMut<NextState<MyAppState>>,
) {
    for event in events.read() {
        let content = event.message();
        info!("Start game for lobby {}", content.lobby_id);
        next_state.set(MyAppState::Game);
    }
}
