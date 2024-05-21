use crate::asset_loader_plugin::MyAssets;
use bevy::utils::HashMap;
use bevy::{
    gltf::Gltf,
    prelude::*,
    render::{mesh::skinning::SkinnedMesh, view::NoFrustumCulling},
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

// Marker component that tell me which entities are scenes being loaded from asset_loader
#[derive(Component, Debug)]
pub struct SceneName(pub String);

// Quick way of acessing animation.
#[derive(Resource)]
pub struct Animations(pub HashMap<String, Handle<AnimationClip>>);

// Marker resource that tell me the name of the entity of the entity that I am acessing
// Good filter and debugger
// Also good acessor better than scene name that only lets me filter one sole entity being the guy the skeleton
#[derive(Resource)]
pub struct SceneEntitiesByName(pub HashMap<String, Entity>);

// This occcurs after the assets were loaded
// But basically this guy spawns and adds extra context to our scenes
// Also fills some resources
pub fn spawn_scenes(
    mut commands: Commands,
    mut next_state: ResMut<NextState<StateSpawnScene>>,
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    let mut scene_entities_by_name = HashMap::new();
    let mut animations = HashMap::new();

    for (file_name, gltf_handle) in &asset_pack.gltf_files {
        // Grabbing the handle for the gltf and spawning it
        if let Some(gltf) = assets_gltf.get(gltf_handle) {
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

            for named_animation in gltf.named_animations.iter() {
                println!("Insert anim {}", named_animation.0);
                // Just transforming the named animations into a hashmap
                animations.insert(
                    named_animation.0.clone(),
                    gltf.named_animations[named_animation.0].clone(),
                );
            }
        }
    }
    commands.insert_resource(SceneEntitiesByName(scene_entities_by_name));
    commands.insert_resource(Animations(animations));

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
