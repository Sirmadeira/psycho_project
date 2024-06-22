use bevy::utils::HashMap;
use bevy::{
    gltf::Gltf,
    prelude::*,
    render::{mesh::skinning::SkinnedMesh, view::NoFrustumCulling},
};
use regex::Regex;

use super::{assemble_parts::get_main_skeleton_bones_and_armature, Attachments};

use crate::asset_loader_plugin::MyAssets;
use crate::mod_char_plugin::{
    assemble_parts::attach_part_to_main_skeleton, lib::ConfigModularCharacters, AmountPlayers,
    Animations, Skeleton, StateSpawnScene,
};

//
pub fn spawn_skeleton_and_attachments(
    mut commands: Commands,
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    amount_players: Res<AmountPlayers>,
    modular_config: Res<ConfigModularCharacters>,
) {
    for number_of_player in 1..=amount_players.quantity {
        let mut skeleton_entity_id: Option<Entity> = None;
        let mut weapons: Vec<Option<Entity>> = Vec::new();
        let mut visuals: Vec<Option<Entity>> = Vec::new();

        let max_len = std::cmp::max(
            modular_config.weapons_to_be_attached.len(),
            modular_config.visuals_to_be_attached.len(),
        );

        for i in 0..max_len {
            for (file_name, gltf_handle) in &asset_pack.gltf_files {
                let gltf = assets_gltf.get(gltf_handle).expect("GLTF to have GLTF");

                // Check and spawn the skeleton
                if skeleton_entity_id.is_none() {
                    if Regex::new(r"(?i)skeleton").unwrap().is_match(file_name) {
                        skeleton_entity_id = Some(
                            commands
                                .spawn((
                                    SceneBundle {
                                        scene: gltf.named_scenes["Scene"].clone(),
                                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                                        ..Default::default()
                                    },
                                    Name::new(format!(
                                        "{}_{}",
                                        &file_name[0..file_name.len() - 4],
                                        number_of_player
                                    )),
                                    Skeleton,
                                ))
                                .id(),
                        );
                    }
                }

                // Check and spawn the weapon
                if let Some(wep) = modular_config.weapons_to_be_attached.get(i) {
                    if Regex::new(&format!(r"(?i){}", wep))
                        .unwrap()
                        .is_match(file_name)
                    {
                        weapons.push(Some(
                            commands
                                .spawn((
                                    SceneBundle {
                                        scene: gltf.named_scenes["Scene"].clone(),
                                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                                        ..Default::default()
                                    },
                                    Name::new(format!(
                                        "{}_{}",
                                        &file_name[0..file_name.len() - 4],
                                        number_of_player
                                    )),
                                ))
                                .id(),
                        ));
                    }
                }

                // Check and spawn the visual
                if let Some(vis) = modular_config.visuals_to_be_attached.get(i) {
                    if Regex::new(&format!(r"(?i){}", vis))
                        .unwrap()
                        .is_match(file_name)
                    {
                        visuals.push(Some(
                            commands
                                .spawn((
                                    SceneBundle {
                                        scene: gltf.named_scenes["Scene"].clone(),
                                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                                        ..Default::default()
                                    },
                                    Name::new(format!(
                                        "{}_{}",
                                        &file_name[0..file_name.len() - 4],
                                        number_of_player
                                    )),
                                ))
                                .id(),
                        ));
                    }
                }
            }
        }

        if let Some(skeleton_entity_id) = skeleton_entity_id {
            // Attach entities to the skeleton
            commands.entity(skeleton_entity_id).insert(Attachments {
                weapons,
                visual: visuals,
            });
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

pub fn attach_to_skeletons(
    q_1: Query<(Entity, &Attachments), With<Attachments>>,
    children_entities: Query<&Children>,
    names: Query<&Name>,
    mut commands: Commands,
) {
    // TODO - MAKE PARENT ACCORDING TO SKELETON
    for (skeleton_entity_id, attachment) in q_1.iter() {
        // Get sub children
        let skeleton_bones =
            get_main_skeleton_bones_and_armature(&children_entities, &names, &skeleton_entity_id);

        // Get primary weapon
        let primary_weapon = attachment.weapons[0].expect("TO have at least one weapon");
        let handle_bone = skeleton_bones
            .get("mixamorig:Handle")
            .expect("Skellie to have bone");

        commands.entity(primary_weapon).set_parent(*handle_bone);
        // Attach visual according to the bone
        let visual_to_insert = attachment.visual[0].expect("TO have at least one visual");
        attach_part_to_main_skeleton(
            &mut commands,
            &children_entities,
            &names,
            &visual_to_insert,
            &skeleton_bones,
        );
    }
}

pub fn disable_culling_for_skinned_meshes(
    mut commands: Commands,
    skinned: Query<Entity, Added<SkinnedMesh>>,
) {
    for entity in &skinned {
        commands.entity(entity).insert(NoFrustumCulling);
    }
}
