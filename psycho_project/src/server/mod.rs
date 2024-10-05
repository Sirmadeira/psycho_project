use crate::shared::protocol::lobby_structs::{Lobbies, SearchMatch, StopSearch};
use crate::shared::protocol::player_structs::{PlayerBundleMap, PlayerVisuals, SaveVisual};
use bevy::prelude::*;
use lightyear::server::events::*;
mod essentials;
use bincode::{deserialize_from, serialize_into};
use std::fs::File;
use std::io::{BufReader, BufWriter};

use self::essentials::*;

pub struct ExampleServerPlugin;

impl Plugin for ExampleServerPlugin {
    fn build(&self, app: &mut App) {
        // Initializing resources
        app.init_resource::<Lobbies>();
        app.init_resource::<PlayerAmount>();
        app.init_resource::<PlayerEntityMap>();

        // Debug registering
        app.register_type::<Lobbies>();
        app.register_type::<PlayerBundleMap>();
        app.register_type::<PlayerStateConnection>();
        app.register_type::<PlayerVisuals>();

        // Do this if adjustment were made in main struct
        // app.add_systems(Startup, create_save_files);

        // Init server in head mode
        app.add_systems(Startup, read_save_files);

        // Listeners
        app.add_systems(Update, listener_save_visuals);
        app.add_systems(Update, listener_search_match);
        app.add_systems(Update, listener_stop_search);

        //Self made plugins

        app.add_plugins(EssentialsPlugin);
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

// Responsible for saving player info
fn listener_save_visuals(
    mut events: EventReader<MessageEvent<SaveVisual>>,
    mut player_map: ResMut<PlayerBundleMap>,
) {
    for event in events.read() {
        let client_id = event.context();

        info!("Grabbing player visuals and body part to change from client");
        let player_visuals = event.message().0.clone();

        info!("Saving player info {}", client_id);

        if let Some(player_bundle) = player_map.0.get_mut(client_id) {
            info!("Found it is bundle and changing visual  for what client said");
            player_bundle.visuals = player_visuals;

            info!("Saving this bundle {:?}", player_bundle);
            save_file(player_map.clone());
        } else {
            error!("Something went wrong in grabing this id info in server");
        }
    }
}

// Responsible for searching for match
fn listener_search_match(
    mut events: EventReader<MessageEvent<SearchMatch>>,
    player_entity_map: Res<PlayerEntityMap>,
    mut online_state: Query<&mut PlayerStateConnection>,
) {
    for event in events.read() {
        let client_id = event.context();

        let player_entity = player_entity_map
            .0
            .get(client_id)
            .expect("To find player in map when searching for his player state");

        let mut on_state = online_state
            .get_mut(*player_entity)
            .expect("For online player to have player state component");

        *on_state = PlayerStateConnection {
            online: true,
            searching: true,
            in_game: false,
        }
    }
}

// Responsible for stop searhcing for match
fn listener_stop_search(
    mut events: EventReader<MessageEvent<StopSearch>>,
    player_entity_map: Res<PlayerEntityMap>,
    mut online_state: Query<&mut PlayerStateConnection>,
) {
    for event in events.read() {
        let client_id = event.context();

        let player_entity = player_entity_map
            .0
            .get(client_id)
            .expect("To find player in map when searching for his player state");

        let mut on_state = online_state
            .get_mut(*player_entity)
            .expect("For online player to have player state component");

        *on_state = PlayerStateConnection {
            online: true,
            searching: false,
            in_game: false,
        }
    }
}
