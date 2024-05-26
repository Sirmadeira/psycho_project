use bevy::prelude::*;

mod assemble_parts;
mod form_colliders;
mod link_animations;
mod run_animations;
mod spawn_scenes;

use self::{
    spawn_scenes::*,
    assemble_parts::assemble_parts, 
    link_animations::link_animations,
    run_animations::run_animations,
};

use crate::asset_loader_plugin::AssetLoaderState;

pub struct ModCharPlugin;

impl Plugin for ModCharPlugin {
    fn build(&self, app: &mut App) {
        // Loads scenes and spawn handles
        app.add_systems(OnEnter(AssetLoaderState::Done), (spawn_scenes,spawn_animation_handle,spawn_morph_data_handle));
        // Will only load after we finished loading the assets
        app.init_state::<StateSpawnScene>();
        app.add_systems(
            OnEnter(StateSpawnScene::Spawned),
            (
                disable_culling_for_skinned_meshes,
                link_animations,
            ),
        );
        app.add_systems(
            OnEnter(StateSpawnScene::Done),
            (run_animations, assemble_parts),
        );
        // app.add_systems(OnEnter(StateSpawnScene::FormingPhysics), spawn_colliders);
        // This guys is gonna run infinetely
        // app.add_systems(
        //     PostUpdate,
        //     col_follow_animation
        //         .run_if(in_state(StateSpawnScene::FormingPhysics))
        //         .after(TransformSystem::TransformPropagate),
        // );
    }
}
