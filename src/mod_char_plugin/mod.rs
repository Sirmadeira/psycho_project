use bevy::prelude::*;

mod spawn_scenes;
mod scene_tree;
mod show_joints;

use self::{spawn_scenes::*,
        scene_tree::scene_tree,
        show_joints::cubes_names_on_joints};

use crate::asset_loader_plugin::AssetLoaderState;

pub struct ModCharPlugin;

impl Plugin for ModCharPlugin {
    fn build(&self, app: &mut App) {
        // Will only load after we finished loading the assets
        app.init_state::<StateSpawnScene>();
        app.add_systems(OnEnter(AssetLoaderState::Done), spawn_scenes);
        app.add_systems(OnEnter(StateSpawnScene::Spawned), (disable_culling_for_skinned_meshes,scene_tree,cubes_names_on_joints));
    }
}
