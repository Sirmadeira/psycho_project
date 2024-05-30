use crate::asset_loader_plugin::MyAssets;
use bevy::utils::HashMap;
use bevy::{
    gltf::Gltf,
    prelude::*,
};

// Tell me in which state the scene is
#[derive(States, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub enum StateSpawnScene {
    #[default]
    Spawning,
    Spawned,
    Done,
    FormingPhysics,
}

// Marker component tells me the scene name or the filename in this case
#[derive(Component, Debug)]
pub struct SceneName(pub String);

// Marker resource that tell me the name of the scene and the scene entity that I am acessing
#[derive(Resource)]
pub struct SceneEntitiesByName(pub HashMap<String, Entity>);

// Quick way of acessing animation data. I know  there is gltf animation but i dont want to call my asset pack all the time
#[derive(Resource)]
pub struct Animations(pub HashMap<String, Handle<AnimationClip>>);

// This occcurs after the assets were loaded
pub fn spawn_scenes(
    mut commands: Commands,
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    // Acessing scenes by name
    let mut scene_entities_by_name = HashMap::new();

    for (file_name, gltf_handle) in &asset_pack.gltf_files {
        // Grabbing the handle for the gltf and spawning it
        if let Some(gltf) = assets_gltf.get(gltf_handle) {
            // Spawning the scenes, with a specific scene name struct
            let entity_commands = commands.spawn((
                SceneBundle {
                    scene: gltf.named_scenes["Scene"].clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..Default::default()
                },
                SceneName(file_name.clone()),
            ));

            let entity = entity_commands.id();
            scene_entities_by_name.insert(file_name.clone(), entity);
        }
    }
    commands.insert_resource(SceneEntitiesByName(scene_entities_by_name));
}

// Forming the handle for meshes morph data
pub fn spawn_animation_handle(
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<StateSpawnScene>>,
) {
    let mut animations = HashMap::new();

    for gltf_handle in asset_pack.gltf_files.values() {
        if let Some(gltf) = assets_gltf.get(gltf_handle) {
            // Forming the handle for animations
            for named_animation in gltf.named_animations.iter() {
                println!("Insert anim {}", named_animation.0);
                animations.insert(
                    named_animation.0.clone(),
                    gltf.named_animations[named_animation.0].clone(),
                );
            }
        }
    }
    commands.insert_resource(Animations(animations));
    next_state.set(StateSpawnScene::Spawned);
}
