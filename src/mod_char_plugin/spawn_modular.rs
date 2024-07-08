use std::collections::HashMap;
use bevy::{
    prelude::*,
    gltf::Gltf,
    utils::Duration,
    render::{mesh::skinning::SkinnedMesh, view::NoFrustumCulling}};
use regex::Regex;

use super::{assemble_parts::get_main_skeleton_bones_and_armature, Attachments};
use crate::load_assets_plugin::MyAssets;
use crate::mod_char_plugin::{assemble_parts::attach_part_to_main_skeleton, lib::ConfigModularCharacters, AmountPlayers,Skeleton, StateSpawnScene,Animations};

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

        // This values is basedc on the maximum value in the two given vectors
        for i in 0..max_len {
            for (file_name, gltf_handle) in &asset_pack.gltf_files {
                let gltf = assets_gltf.get(gltf_handle).expect("My asset pack to have GLTF");
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
        // Spawn the base entity
        if let Some(skeleton_entity_id) = skeleton_entity_id {
            // Attach entities to the skeleton
            commands.entity(skeleton_entity_id)
            .insert(Attachments {
                weapons,
                visual: visuals,
            });
        }
    }
}

// Creates animation graph for each player and add it is clips to it
pub fn spawn_animations_graphs(amount_players: Res<AmountPlayers>,
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut assets_animation_graph: ResMut<Assets<AnimationGraph>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<StateSpawnScene>>,
    ){

    for number_of_player in 1..=amount_players.quantity{

        // Creating graphs according to amount of player
        let mut graph = AnimationGraph::new();

        // Node with a string name
        let mut named_nodes = HashMap::new();

        // Using bevy asset loader to easily acess my assets
        for (_, gltf_handle) in &asset_pack.gltf_files{
            let gltf = assets_gltf.get(gltf_handle).expect("My asset pack to have GLTF");

            // Creating named nodes
            for (name_animation,animation_clip) in gltf.named_animations.iter(){
                // Returns animations node
                let node = graph.add_clip(animation_clip.clone(),1.0, graph.root);
                // Creating named node
                named_nodes.insert(name_animation.to_string(), node);
                println!("Current available animations are {} for player {}",name_animation, number_of_player);
            }

        }   

        // Adding animation graph to assets
        let anim_graph = assets_animation_graph.add(graph);

        // Formulating resource that tells me what is the name of the animation in a node and it is animation graph
        commands.insert_resource(Animations{
            named_nodes:named_nodes,
            animation_graph:anim_graph.clone()
        });
    }
    next_state.set(StateSpawnScene::Spawned);
}


pub fn attach_to_skeletons(
    q_1: Query<(Entity, &Attachments), With<Attachments>>,
    children_entities: Query<&Children>,
    names: Query<&Name>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<StateSpawnScene>>,
) {
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
    next_state.set(StateSpawnScene::Done)
}

pub fn disable_culling_for_skinned_meshes(
    mut commands: Commands,
    skinned: Query<Entity, Added<SkinnedMesh>>,
) {
    for entity in &skinned {
        commands.entity(entity).insert(NoFrustumCulling);
    }
}



pub fn test_animations(    mut commands: Commands,
    animations: Res<Animations>,   
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,){

    // Each skinned mesh already  comes with a prespawned animation player struct
    for (entity, mut player) in &mut players {
        let mut transitions = AnimationTransitions::new();
        transitions.play(&mut player, animations.named_nodes["RightAttack"], Duration::ZERO).repeat();
        // Display the transitions of the current entity
        commands
        .entity(entity)
        .insert(animations.animation_graph.clone())
        .insert(transitions);

    }
}
