//! RESPONSIBILITIES - HANDLES ALL MODULAR CHARACTERS CREATIONS

use crate::shared::protocol::player_structs::*;
use bevy::prelude::*;

use lightyear::client::events::*;

use super::MyAppState;

pub struct CreateCharPlugin;

impl Plugin for CreateCharPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MyAppState::Game), naming_player);
    }
}

/// Example system to handle ComponentInsertEvent events
pub(crate) fn naming_player(
    player_ids: Query<(Entity, &PlayerId), With<PlayerId>>,
    mut commands: Commands,
) {
    for (player, player_id) in player_ids.iter() {
        info!("Naming characters according to their client for easy to find in debugger");
        let name = Name::new(format!("Player {}", player_id.0));
        commands.entity(player).insert(name);
    }
}

// TODO - UI FOR THIS for now client only sends the default value
pub(crate) fn insert_visuals(player_id: Query<Entity, With<PlayerId>>) {}

// This will spawn our main characters according TO THE AMOUNT OF ENTITIES, IN LOBBY. TODO LOBBY
// pub(crate) fn spawn_character(
//     player: Query<Entity, With<Predicted>>,
//     client_collection: Res<ClientCharCollection>,
//     assets_gltf: Res<Assets<Gltf>>,
//     mut commands: Commands,
// ) {
//     for _ in player.iter() {
//         info!("All players being created");
//         for (file_name, han_gltf) in &client_collection.gltf_files {
//             if file_name.contains("character_mesh") {
//                 // Loading gltf from asset_server
//                 let gltf_scene = assets_gltf
//                     .get(han_gltf)
//                     .expect("The handle in server to be loaded");

//                 // Grabbng mesh
//                 let player_mesh = SceneBundle {
//                     scene: gltf_scene.named_scenes["Scene"].clone(),
//                     transform: Transform::from_xyz(0.0, 0.0, 0.0),
//                     ..Default::default()
//                 };

//                 commands.spawn(player_mesh);
//             }
//         }
//     }
// }
