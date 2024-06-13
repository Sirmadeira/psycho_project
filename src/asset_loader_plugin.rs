use bevy::gltf::Gltf;
use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::prelude::*;

// State - That tell us when we are loading our character
#[derive(States, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub enum AssetLoaderState {
    #[default]
    Loading,
    Done,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AssetLoaderState>().add_loading_state(
            LoadingState::new(AssetLoaderState::Loading)
                .continue_to_state(AssetLoaderState::Done)
                .load_collection::<MyAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    // Insert in this path which asset you want to load
    // He will be loaded with multiple handles and a usefull hashmap to identify it in our code
    #[asset(
        paths("skeleton_female.glb", "rigge_female.glb"),
        collection(typed, mapped)
    )]
    pub gltf_files: HashMap<String, Handle<Gltf>>,
}
