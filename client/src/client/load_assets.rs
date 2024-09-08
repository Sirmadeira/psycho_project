use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;
use lightyear::prelude::client::Predicted;

pub struct LoadingAssetsPlugin;
use crate::client::MyClientState;

impl Plugin for LoadingAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MyClientState>().add_loading_state(
            LoadingState::new(MyClientState::LoadingAssets)
                .continue_to_state(MyClientState::Loaded)
                .load_collection::<ClientCollection>(),
        );
        app.add_systems(OnEnter(MyClientState::Loaded), spawn_character);
    }
}

// Resource for easily acessing client based assets, which are mostly things like character world and so on. Each field in the connect is gonna be associate with something.
#[derive(AssetCollection, Resource)]
pub struct ClientCollection {
    #[asset(paths("character_mesh.glb"), collection(typed, mapped))]
    pub gltf_files: HashMap<String, Handle<Gltf>>,
}

#[derive(Resource, Reflect)]
pub struct ConfigModularCharacters {
    pub visual_to_attach: Vec<String>,
    pub weapons_to_attache: Vec<String>,
}

impl Default for ConfigModularCharacters {
    fn default() -> Self {
        ConfigModularCharacters {
            visual_to_attach: vec!["character_mesh".to_string()],
            weapons_to_attache: vec!["katana".to_string()],
        }
    }
}

// This will spawn our main characters according TO THE AMOUNT OF ENTITIES, IN LOBBY. TODO LOBBY
pub(crate) fn spawn_character(
    player: Query<Entity, With<Predicted>>,
    client_collection: Res<ClientCollection>,
    assets_gltf: Res<Assets<Gltf>>,
    mut commands: Commands,
) {
    for _ in player.iter() {
        info!("All players being created");
        for (file_name, han_gltf) in &client_collection.gltf_files {
            if file_name.contains("character_mesh") {
                // Loading gltf from asset_server
                let gltf_scene = assets_gltf
                    .get(han_gltf)
                    .expect("The handle in server to be loaded");

                // Grabbng mesh
                let player_mesh = SceneBundle {
                    scene: gltf_scene.named_scenes["Scene"].clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..Default::default()
                };

                commands.spawn(player_mesh);
            }
        }
    }
}
