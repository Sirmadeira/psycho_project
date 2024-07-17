
use bevy::utils::HashMap;
use bevy::{
    animation::AnimationTarget,
    gltf::Gltf,
    prelude::*,
    render::{mesh::skinning::SkinnedMesh, view::NoFrustumCulling},

};
use regex::Regex;

use crate::load_assets_plugin::MyAssets;
use crate::mod_char::{lib::*, AmountPlayers, Skeleton, StateSpawnScene};

use super::helpers::{collect_bones, find_child_with_name_containing};

// Spawn main skeleton and his attachments/visual bones. According to given scene name in resource configs
pub fn spawn_skeleton_and_attachments(
    mut commands: Commands,
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    amount_players: Res<AmountPlayers>,
    modular_config: Res<ConfigModularCharacters>,
    mut next_state: ResMut<NextState<StateSpawnScene>>,
) {
    for number_of_player in 1..=amount_players.quantity {
        let mut skeleton_entity_id: Option<Entity> = None;
        let mut weapons: Vec<Option<Entity>> = Vec::new();
        let mut visuals: Vec<Option<Entity>> = Vec::new();

        let max_len = std::cmp::max(
            modular_config.weapons_to_be_attached.len(),
            modular_config.visuals_to_be_attached.len(),
        );

        // This values is basedc on the maximum value in the two given vectors
        for i in 0..max_len {
            for (file_name, gltf_handle) in &asset_pack.gltf_files {
                let gltf = assets_gltf
                    .get(gltf_handle)
                    .expect("My asset pack to have GLTF");
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
                                    Visual,
                                ))
                                .id(),
                        ));
                    }
                }
            }
        }
        // Spawn the base entity
        if let Some(skeleton_entity_id) = skeleton_entity_id {
            // Attach entities to the skeleton
            commands.entity(skeleton_entity_id).insert(Attachments {
                weapons,
                visual: visuals,
            });
        }
    }
    next_state.set(StateSpawnScene::Spawned);
}


// Transfer the animations to all the visual bones
pub fn transfer_animation(
    skeletons: Query<Entity, With<Skeleton>>,
    visuals: Query<Entity, With<Visual>>,
    animation_target: Query<&AnimationTarget>,
    children_entities: Query<&Children>,
    names: Query<&Name>,
    mut commands: Commands,
) {
    for skeleton in skeletons.iter() {
        let old_entity =
            find_child_with_name_containing(&children_entities, &names, &skeleton, "Armature")
                .expect("Armature 1");

        let mut old_bones = HashMap::new();
        collect_bones(&children_entities, &names, &old_entity, &mut old_bones);

        for visual in visuals.iter() {
            let new_entity =
                find_child_with_name_containing(&children_entities, &names, &visual, "Armature")
                    .expect("Armature 2");

            commands.entity(new_entity).insert(AnimationPlayer::default());

            let mut new_bones = HashMap::new();
            collect_bones(&children_entities, &names, &new_entity, &mut new_bones);

            for (name, entity) in old_bones.iter() {
                let old_animation_target = animation_target.get(*entity).expect("To have target");

                let new_match_entity = new_bones.get(name).expect("To have matching bone");

                commands
                    .entity(*new_match_entity)
                    .insert(AnimationTarget{id: old_animation_target.id,player:new_entity});
            }
        }
    }
}

// Constructs final skeleton entity - Makes visual armatures child of it and parents  weapons correctly. Also despawn old armatures
pub fn make_end_entity(
    skeleton: Query<(Entity, &Attachments), With<Skeleton>>,
    children_entities: Query<&Children>,
    names: Query<&Name>,
    mut transforms: Query<&mut Transform>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<StateSpawnScene>>,
) {
    for (skeleton, attachments) in skeleton.iter() {
        // This isnt despawned earlier because of apply_deffered
        let old_base_armature =
            find_child_with_name_containing(&children_entities, &names, &skeleton, "Armature")
                .expect("Old armature");

        commands.entity(old_base_armature).despawn_recursive();

        for attachment in attachments.visual.iter() {
            if let Some(visual_attachment) = attachment {
                commands
                    .entity(*visual_attachment)
                    .set_parent_in_place(skeleton);

                for attachment in attachments.weapons.iter() {
                    if let Some(weapon_attachment) = attachment {
                        if let Some(handle_gun) = find_child_with_name_containing(
                            &children_entities,
                            &names,
                            &visual_attachment,
                            "Handle",
                        ) { 

                            // Adjusting transform
                            commands.entity(*weapon_attachment).set_parent(handle_gun);

                            let mut transform = transforms.get_mut(*weapon_attachment).expect("Transform to apply offset");

                            let amount:f32 = -180.0;
                            transform.rotation = Quat::from_axis_angle(Vec3::X, amount.to_radians() );

                        } else {
                            println!("The visual bone {} didn't have a handle", visual_attachment);
                        }
                    }
                }
            }
        }
    }
    next_state.set(StateSpawnScene::Done);
}


// Debugger function in animations
pub fn disable_culling_for_skinned_meshes(
    mut commands: Commands,
    skinned: Query<Entity, Added<SkinnedMesh>>,
) {
    for entity in &skinned {
        commands.entity(entity).insert(NoFrustumCulling);
    }
}
