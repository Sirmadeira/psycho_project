use bevy::prelude::*;

mod spawn_scenes;
mod scene_tree;
mod show_joints;
mod link_animations;
mod run_animations;
mod assemble_parts;

use self::{spawn_scenes::*,
        link_animations::link_animations,
        scene_tree::scene_tree,
        show_joints::cubes_names_on_joints,
        run_animations::run_animations,
        assemble_parts::assemble_parts};

use crate::asset_loader_plugin::AssetLoaderState;

pub struct ModCharPlugin;

impl Plugin for ModCharPlugin {
    fn build(&self, app: &mut App) {
        // Will only load after we finished loading the assets
        app.init_state::<StateSpawnScene>();
        app.add_systems(OnEnter(AssetLoaderState::Done), spawn_scenes);
        app.add_systems(OnEnter(StateSpawnScene::Spawned), (disable_culling_for_skinned_meshes,scene_tree,link_animations));
        app.add_systems(OnEnter(StateSpawnScene::Done), (run_animations,assemble_parts).chain());
    }
}
