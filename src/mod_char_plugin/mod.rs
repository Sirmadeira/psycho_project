use bevy::prelude::*;
use link_animations::AnimationEntityLink;
use spawn_modular::SceneName;

// Making thme public just in case i need to query a specific component or resource for future logic
pub mod assemble_parts;
pub mod helpers;
pub mod lib;
pub mod link_animations;
pub mod spawn_modular;

use self::{
    lib::*, link_animations::link_animations, spawn_modular::*,
};

use crate::asset_loader_plugin::AssetLoaderState;

// This plugin creates the character
pub struct ModCharPlugin;

impl Plugin for ModCharPlugin {
    fn build(&self, app: &mut App) {
        // Debuging
        app.register_type::<AnimationEntityLink>();
        app.register_type::<SceneName>();
        // Config resources
        app.insert_resource(ConfigModularCharacters {
            quantity: 2,
            visuals_to_be_attached: vec![String::from("rigge_female")],
            weapons: vec![String::from("katana")],
        });
        // Loads scenes and spawn handles
        app.add_systems(
            OnEnter(AssetLoaderState::Done),
            (spawn_scenes, spawn_animation_handle).chain(),
        );
        // Avoid bug in skinnes meshes
        app.add_systems(Update, disable_culling_for_skinned_meshes);
        // Will only load after we finished loading the assets
        app.init_state::<StateSpawnScene>();
        app.add_systems(OnEnter(StateSpawnScene::Spawned), link_animations);

    }
}
