use bevy::prelude::*;
use bevy::transform::TransformSystem;

mod assemble_parts;
mod form_colliders;
mod link_animations;
mod run_animations;
mod spawn_scenes;

use self::{
    assemble_parts::assemble_parts, form_colliders::*, link_animations::link_animations,
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
        app.register_type::<Offset>();
        app.add_systems(
            OnEnter(StateSpawnScene::Spawned),
            (disable_culling_for_skinned_meshes, link_animations),
        );
        app.add_systems(
            OnEnter(StateSpawnScene::Done),
            (run_animations, assemble_parts),
        );
        app.add_systems(OnEnter(StateSpawnScene::FormingPhysics), spawn_colliders);
        app.add_systems(
            PostUpdate,
            col_follow_animation
                .run_if(in_state(StateSpawnScene::FormingPhysics))
                .after(TransformSystem::TransformPropagate),
        );
        app.add_systems(
            PostUpdate,
            hard_colliders_look_at
                .run_if(in_state(StateSpawnScene::FormingPhysics))
                .after(TransformSystem::TransformPropagate),
        );
    }
}
