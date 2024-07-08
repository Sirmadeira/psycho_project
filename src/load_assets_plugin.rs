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

// A way of acessing my gltfs via their file names, and them handle to their gltfs
// This exist because assets needs to be acessed via handles so this guy can be pretty handy it avoid having to acess and interact through a bunch of unecessary assets
#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    #[asset(
        paths("skeleton.glb", "rigge_female.glb", "katana.glb"),
        collection(typed, mapped)
    )]
    pub gltf_files: HashMap<String, Handle<Gltf>>,
}
