use regex::Regex;
use bevy::{ gltf::Gltf, prelude::*, render::{mesh::skinning::SkinnedMesh, view::NoFrustumCulling}
};
use bevy::utils::HashMap;
use crate::asset_loader_plugin::MyAssets;
use crate::mod_char_plugin::{lib::ConfigModularCharacters,assemble_parts::attach_part_to_main_skeleton};

use super::assemble_parts::get_main_skeleton_bones_and_armature;

// Tell me in which state the scene is
#[derive(States, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub enum StateSpawnScene {
    #[default]
    Spawning,
    Spawned,
    Done,
}

//Marker component that tells me which one is a visual scene and which one is the skeleton entity
#[derive(Component, Debug)]
pub struct Skeleton;

// Marker component that tell me which ones are weapons
#[derive(Component, Debug)]
pub struct Weapon;

// Marker component that tell me which ones are weapons
#[derive(Component, Debug)]
pub struct Visual;

// Marker component tells me the scene name or the filename in this case
#[derive(Component, Reflect, Debug)]
pub struct SceneName(pub String);

// Quick way of acessing animation data. I know  there is gltf animation but i dont want to call my asset pack all the time
#[derive(Resource)]
pub struct Animations(pub HashMap<String, Handle<AnimationClip>>);

// 
pub fn spawn_scenes(
    mut commands: Commands,
    // Every single asset
    asset_pack: Res<MyAssets>,
    // Pointer to asset handle
    assets_gltf: Res<Assets<Gltf>>,
    modular_config: Res<ConfigModularCharacters>,
    children_entities: Query<&Children>,
    names: Query<&Name>,
) {
    // Spawning skeleton amount according to modular config
    // Skeleton base entity one to many relation with weapong and visuals
    for number_of_player in 1..=modular_config.quantity {
        let mut skeleton_entity_id: Option<Entity> = None; 
        let mut weapons:  Vec<Option<Entity>> = vec![None; modular_config.weapons.len()];
        let mut visuals:  Vec<Option<Entity>> = vec![None; modular_config.visuals_to_be_attached.len()];

        for (file_name, gltf_handle) in &asset_pack.gltf_files {
            let gltf = assets_gltf.get(gltf_handle).expect("GLTF to have GLTF");
            // Handles bone entities
            let pat = Regex::new(r"(?i)skeleton").unwrap();
            if pat.is_match(&file_name) {
                println!("{}",file_name);
                skeleton_entity_id = Some(commands.spawn((
                    SceneBundle {
                        scene: gltf.named_scenes["Scene"].clone(),
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                        ..Default::default()
                    },
                    SceneName(file_name.clone() + &format!("{}", number_of_player)),
                    Skeleton,
                )).id());
            }
            for wep in &modular_config.weapons{
                let pat2 = Regex::new(&format!(r"(?i){}", wep)).unwrap();
                if pat2.is_match(&file_name){
                    weapons.push(Some(commands.spawn((
                        SceneBundle {
                            scene: gltf.named_scenes["Scene"].clone(),
                            transform: Transform::from_xyz(0.0, 0.0, 0.0),
                            ..Default::default()
                        },
                        SceneName(file_name.clone() + &format!("{}", number_of_player)),
                        Weapon,
                    )).id()));
                }
            }
            for vis in &modular_config.visuals_to_be_attached{
                let pat3 = Regex::new(&format!(r"(?i){}", vis)).unwrap();
                if pat3.is_match(&file_name){
                    visuals.push(Some(commands.spawn((
                        SceneBundle {
                            scene: gltf.named_scenes["Scene"].clone(),
                            transform: Transform::from_xyz(0.0, 0.0, 0.0),
                            ..Default::default()
                        },
                        SceneName(file_name.clone() + &format!("{}", number_of_player)),
                        Visual,
                    )).id()));
                }
            }
        }

        if let Some(skeleton_entity_id) = skeleton_entity_id {

            for i in DescendantIter::new(&children_entities,skeleton_entity_id){
                println!("{}",names.get(i).unwrap().to_string());
            }
            // // Get skeleton bone
            // let skeleton_bones = get_main_skeleton_bones_and_armature(&children_entities, &names, &skeleton_entity_id);
            // // Get primary weapon
            // let primary_weapon = weapons[0].expect("TO have at least one weapon");
            // let handle_bone = skeleton_bones
            // .get("mixamorig:Handle")
            // .expect("Skellie to have bone");
            // commands.entity(primary_weapon).set_parent(*handle_bone);
            // // Attach visual according to the bone
            // let visual_to_insert = visuals[0].expect("TO have at least one visual");
            // attach_part_to_main_skeleton(
            //     &mut commands,
            //     &children_entities,
            //     &names,
            //     &visual_to_insert,
            //     &skeleton_bones,
            // );
        }
    }
}



// Forming the handle for meshes morph data
pub fn spawn_animation_handle(
    mut commands: Commands,
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
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

pub fn disable_culling_for_skinned_meshes(
    mut commands: Commands,
    skinned: Query<Entity, Added<SkinnedMesh>>,
) {
    for entity in &skinned {
        commands.entity(entity).insert(NoFrustumCulling);
    }
}
