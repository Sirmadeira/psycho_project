use crate::shared::protocol::player_structs::PlayerBundleMap;
use bevy::prelude::*;
use bincode::serialize_into;
use lobby::LobbyPlugin;
use player::PlayerPlugin;
use std::fs::File;
use std::io::BufWriter;
use world::PhysicsWorldPlugin;

mod essentials;
mod lobby;
mod player;
mod world;

use self::essentials::*;

/// Important plugin here you should centralize all systems/plugins that are heavily correlated to server
pub struct ExampleServerPlugin;

impl Plugin for ExampleServerPlugin {
    fn build(&self, app: &mut App) {
        // Do this if adjustment were made in main struct
        // app.add_systems(Startup, create_save_files);

        //Self made plugins
        app.add_plugins(PhysicsWorldPlugin);
        app.add_plugins(EssentialsPlugin);
        app.add_plugins(LobbyPlugin);
        app.add_plugins(PlayerPlugin);
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
