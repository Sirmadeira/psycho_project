use crate::asset_loader_plugin::MyAssets;
use bevy::{
 gltf::Gltf, prelude::*, render::{mesh::skinning::SkinnedMesh, view::NoFrustumCulling}
};

// Tell me in which state the scene is
#[derive(States, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub enum StateSpawnScene {
    #[default]
    Spawning,
    Spawned,
    Done,
}

// Marker component that tell me which entities are scenes being loaded from asset_loader
#[derive(Component,Debug)]
pub struct SceneName(pub String);

// This occcurs after the assets were loaded
pub fn spawn_scenes(
    mut commands: Commands,
    mut next_state: ResMut<NextState<StateSpawnScene>>,
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    for (file_name, gltf_handle) in &asset_pack.gltf_files {
        // Grabbing the handle for the gltf and spawning it
        if let Some(gltf) = assets_gltf.get(gltf_handle) {
            commands.spawn((
                SceneBundle {
                    scene: gltf.named_scenes["Scene"].clone(),
                    ..Default::default()
                },
                SceneName(file_name.clone()),
            ));
        }
    }

    next_state.set(StateSpawnScene::Spawned);
}


// Just here to avoid bug while i build
pub fn disable_culling_for_skinned_meshes(
    mut commands: Commands,
    skinned: Query<Entity, Added<SkinnedMesh>>,
) {
    for entity in &skinned {
        commands.entity(entity).insert(NoFrustumCulling);
    }
}
