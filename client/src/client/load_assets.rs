//! RESPONSIBILITIES - LOAD ALL ASSETS WHEN GAME STARTS
//! Once loaded we will continue to state UI

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;

pub struct LoadingAssetsPlugin;
use crate::client::MyAppState;

impl Plugin for LoadingAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MyAppState>().add_loading_state(
            LoadingState::new(MyAppState::LoadingAssets)
                .continue_to_state(MyAppState::MainMenu)
                .load_collection::<ClientCharCollection>(),
        );
    }
}

// Resource for easily acessing client based assets, which are mostly things like character world and so on. Each field in the connect is gonna be associate with something.
#[derive(AssetCollection, Resource)]
pub struct ClientCharCollection {
    #[asset(
        paths(
            "characters/character_mesh.glb",
            "characters/body.glb",
            "weapons/katana.glb"
        ),
        collection(typed, mapped)
    )]
    pub gltf_files: HashMap<String, Handle<Gltf>>,
}
