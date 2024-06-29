use bevy::gltf::Gltf;
use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::prelude::*;

use crate::MyAppState;


pub struct LoadingAssetsPlugin;

// After loading the assets we go to the main menu
impl Plugin for LoadingAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MyAppState>().add_loading_state(
            LoadingState::new(MyAppState::Loading)
                .continue_to_state(MyAppState::MainMenu)
                .load_collection::<MyAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    // Insert in this path which asset you want to load
    // He will be loaded with multiple handles and a usefull hashmap to identify it in our code
    #[asset(
        paths("skeleton.glb", "rigge_female.glb", "katana.glb"),
        collection(typed, mapped)
    )]
    pub gltf_files: HashMap<String, Handle<Gltf>>,
}
