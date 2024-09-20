use crate::shared::protocol::lobby_structs::Lobbies;
use crate::shared::protocol::player_structs::{
    PlayerBundle, PlayerBundleMap, PlayerLoadout, PlayerVisuals,
};
use bevy::prelude::*;
use lightyear::server::events::*;
mod server_systems;
use bincode::serialize_into;
use std::fs::File;
use std::io::BufWriter;

use self::server_systems::*;

pub struct ExampleServerPlugin;

impl Plugin for ExampleServerPlugin {
    fn build(&self, app: &mut App) {
        // Initializing resources
        app.init_resource::<Lobbies>();
        app.init_resource::<PlayerBundleMap>();
        app.init_resource::<PlayerAmount>();

        // Debug registering
        app.register_type::<Lobbies>();
        app.register_type::<PlayerBundleMap>();
        app.register_type::<PlayerVisuals>();

        app.add_systems(Startup, create_save_files);
        // Initializing sever current has head
        app.add_systems(Startup, (init, start_server));

        // What happens when you connects to server
        app.add_systems(Update, handle_connections);

        // What happens when you disconnect from server
        app.add_systems(Update, handle_disconnections);

        // Creates a lobby
        app.add_systems(Update, create_lobby);

        // Listeners
        app.add_systems(Update, listener_player_loadout);
    }
}

// Responsible for changing the loadout right now it just
fn listener_player_loadout(
    mut events: EventReader<MessageEvent<PlayerLoadout>>,
    mut player_map: ResMut<PlayerBundleMap>,
) {
    for event in events.read() {
        let message = event.message();
        let client_id = event.context();

        info!("Receiveing new player loadout from {}", client_id);

        if let Some(player_bundle) = player_map.0.get_mut(client_id) {
            player_bundle.visuals = message.0.clone();
            info!("Found it is bundle and changing it for what client said");
            let encoded = bincode::serialize(&player_bundle).unwrap();
            let decoded: PlayerBundle = bincode::deserialize(&encoded[..]).unwrap();
            info!("SHow me decoded {:?}", decoded);
        } else {
            error!("Something went worng in grabing this id info in server");
        }
    }
}

// Create server side binencoded files really important in case server goes down and such
fn create_save_files() {
    let mut f =
        BufWriter::new(File::create("./psycho_project/src/server/save_files/foo.bar").unwrap());
    let player_bundle = PlayerBundleMap::default();
    serialize_into(&mut f, &player_bundle).unwrap();
}

fn read_save_files() {}
