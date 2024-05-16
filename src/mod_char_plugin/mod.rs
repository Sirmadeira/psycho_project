use bevy::prelude::*;

mod assemble_parts;
mod form_colliders;
mod link_animations;
mod run_animations;
mod scene_tree;
mod show_joints;
mod spawn_scenes;

use self::{
    assemble_parts::assemble_parts, form_colliders::*, link_animations::link_animations,
    run_animations::run_animations, scene_tree::scene_tree, spawn_scenes::*,
};

use crate::asset_loader_plugin::AssetLoaderState;

pub struct ModCharPlugin;

impl Plugin for ModCharPlugin {
    fn build(&self, app: &mut App) {
        // Will only load after we finished loading the assets
        app.init_state::<StateSpawnScene>();
        app.add_systems(OnEnter(AssetLoaderState::Done), spawn_scenes);
        app.add_systems(
            OnEnter(StateSpawnScene::Spawned),
            (
                disable_culling_for_skinned_meshes,
                scene_tree,
                link_animations,
            ),
        );
        app.add_systems(
            OnEnter(StateSpawnScene::Done),
            (run_animations, assemble_parts,insert_colliders),
        );
    }
}
