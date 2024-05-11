use crate::asset_loader_plugin::MyAssets;
use bevy::{gltf::Gltf, prelude::*};

// THis occcurs after the assets were loaded
pub fn spawn_scenes(
    mut commands : Commands,
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,

)
{
    for (_,gltf_handle) in &asset_pack.gltf_files {

        // Grabbing the handle for the gltf and spawning it
        if let Some(gltf) = assets_gltf.get(gltf_handle) {
            commands.spawn(SceneBundle {
                scene: gltf.named_scenes["Scene"].clone(),
                ..Default::default()
            });
        }
    }
}