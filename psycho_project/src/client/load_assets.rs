//! RESPONSIBILITIES - LOAD ALL ASSETS WHEN GAME STARTS
//! Once loaded we will continue to state UI

use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::prelude::*;

pub struct LoadingAssetsPlugin;
use crate::client::MyAppState;

impl Plugin for LoadingAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CharCollection>();
        app.register_type::<Images>();
        app.add_loading_state(
            LoadingState::new(MyAppState::LoadingAssets)
                .continue_to_state(MyAppState::MainMenu)
                .load_collection::<CharCollection>()
                .load_collection::<Images>(),
        );
    }
}

// Resource for easily acessing client based assets, which are mostly things like character world and so on. Each field in the connect is gonna be associate with something.
#[derive(AssetCollection, Resource, Reflect)]
#[reflect(Resource)]
pub struct CharCollection {
    #[asset(
        paths(
            // Weapons
            "weapons/katana.glb",
            "characters/character_mesh.glb",
            // Heads
            "characters/parts/suit_head.glb",
            "characters/parts/soldier_head.glb",
            // Torsos
            "characters/parts/scifi_torso.glb",
            "characters/parts/soldier_torso.glb",
            // Legs
            "characters/parts/witch_legs.glb",
            "characters/parts/soldier_legs.glb",
            //Skeletons
            "characters/parts/main_skeleton.glb"
        ),
        collection(typed, mapped)
    )]
    pub gltf_files: HashMap<String, Handle<Gltf>>,
}

#[derive(AssetCollection, Resource, Reflect)]
#[reflect(Resource)]
pub struct Images {
    #[asset(path = "images", collection(typed, mapped))]
    pub map: HashMap<String, Handle<Image>>,
}
