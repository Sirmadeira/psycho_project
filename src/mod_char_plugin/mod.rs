use bevy::prelude::*;

// Making thme public just in case i need to query a specific component or resource for future logic
pub mod assemble_parts;
pub mod link_animations;
pub mod run_animations;
pub mod spawn_scenes;
pub mod helpers;

use self::{
    assemble_parts::assemble_parts, link_animations::link_animations,
    run_animations::run_animations, spawn_scenes::*,
};

use crate::asset_loader_plugin::AssetLoaderState;

pub struct ModCharPlugin;

impl Plugin for ModCharPlugin {
    fn build(&self, app: &mut App) {
        // Loads scenes and spawn handles
        app.add_systems(
            OnEnter(AssetLoaderState::Done),
            (spawn_scenes, spawn_animation_handle),
        );
        // Will only load after we finished loading the assets
        app.init_state::<StateSpawnScene>();
        app.add_systems(
            OnEnter(StateSpawnScene::Spawned),
             link_animations,
        );
        app.add_systems(
            OnEnter(StateSpawnScene::HandlingModularity),
            ( assemble_parts,run_animations),
        );
    }
}
