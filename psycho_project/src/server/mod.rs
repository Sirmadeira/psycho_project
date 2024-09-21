use crate::shared::protocol::lobby_structs::Lobbies;
use crate::shared::protocol::player_structs::{PlayerBundleMap, PlayerLoadout, PlayerVisuals};
use bevy::prelude::*;
use lightyear::server::events::*;
mod server_systems;
use bincode::{deserialize_from, serialize_into};
use std::fs::File;
use std::io::{BufReader, BufWriter};

use self::server_systems::*;

pub struct ExampleServerPlugin;

impl Plugin for ExampleServerPlugin {
    fn build(&self, app: &mut App) {
        // Initializing resources
        app.init_resource::<Lobbies>();
        app.init_resource::<PlayerAmount>();

        // Debug registering
        app.register_type::<Lobbies>();
        app.register_type::<PlayerBundleMap>();
        app.register_type::<PlayerVisuals>();

        // app.add_systems(Startup, create_save_files);
        // Initializing sever current has head
        app.add_systems(Startup, (start_server, init, read_save_files));

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

// IF you adjust one of the player bundle sub-structures you will need to run this solely, if not memory alloc error
#[allow(dead_code)]
fn create_save_files() {
    let mut f = BufWriter::new(
        File::create("./psycho_project/src/server/save_files/player_info.bar").unwrap(),
    );
    let player_bundle = PlayerBundleMap::default();
    serialize_into(&mut f, &player_bundle).unwrap();
}

// Overwrites or create new file that will currently store only the player_bundle_map
fn save_file(save_info: PlayerBundleMap) {
    info!("Saving");
    let mut f = BufWriter::new(
        File::create("./psycho_project/src/server/save_files/player_info.bar").unwrap(),
    );
    let player_bundle = &save_info;
    serialize_into(&mut f, &player_bundle).unwrap();
}

// Reads current save files and fill up the resource playerbundlemap each basically gives me all player info
fn read_save_files(mut commands: Commands) {
    let f = BufReader::new(
        File::open("./psycho_project/src/server/save_files/player_info.bar").unwrap(),
    );
    let player_bundle_map: PlayerBundleMap = deserialize_from(f).unwrap();
    info!("Read from save file: {:?}", player_bundle_map);

    commands.insert_resource(player_bundle_map);
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
            info!("Found it is bundle and changing it for what client said");
            player_bundle.visuals = message.0.clone();
            info!("Saving this bundle {:?}", player_bundle);
            save_file(player_map.clone());
        } else {
            error!("Something went wrong in grabing this id info in server");
        }
    }
}
