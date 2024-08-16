use bevy::{animation::AnimationTarget, prelude::*};

use crate::load_assets_plugin::MyAssets;
use crate::treat_animations::lib::*;

use crate::form_modular_char::helpers::find_child_with_name_containing;
use crate::form_modular_char::lib::Skeleton;

// This will mark every single bone in both entities with bone mask struct
pub fn mark_bones(
    mut commands: Commands,
    q_1: Query<Entity, With<Skeleton>>,
    children_entities: Query<&Children>,
    names: Query<&Name>,
) {
    for root_armature in q_1.iter() {
        // Bone you want to start marking it is descendants in this case since I want all upperbody to be blended mask SPINE it is
        let starting_bone =
            find_child_with_name_containing(&children_entities, &names, &root_armature, "Spine_1")
                .expect("To have skeleton bone");
        let mut entities_to_mark = vec![starting_bone];

        for upper_bones in children_entities.iter_descendants(starting_bone) {
            entities_to_mark.push(upper_bones);
        }

        for entity in entities_to_mark {
            commands.entity(entity).insert(BoneMask);
        }
    }
}
pub fn blend_animations(
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut assets_clips: ResMut<Assets<AnimationClip>>,
    mut assets_animation_graph: ResMut<Assets<AnimationGraph>>,
    mut config_blend_animations: ResMut<ConfigBoneMaskedAnimations>,
    masked_bones: Query<&AnimationTarget, With<BoneMask>>,
    not_masked: Query<&AnimationTarget, (With<AnimationTarget>, Without<BoneMask>)>,
    mut commands: Commands,
) {
    let skeleton_handle = asset_pack
        .gltf_files
        .get("skeleton.glb")
        .expect("To have skeleton gltf handle");

    let gltf = assets_gltf
        .get(skeleton_handle)
        .expect("My asset pack to have GLTF");

    // Create animation graph and vec to push nodes
    let mut blended_graph = AnimationGraph::default();
    let mut new_nodes = Vec::new();

    // Checking our resource config and saving the handles
    for blend_animation in config_blend_animations.0.iter_mut() {
        // Fills them up with the clips to be blended
        for (name, handle_clip) in gltf.named_animations.iter() {
            if blend_animation.first_anim == name.to_string() {
                blend_animation.first_anim_clip = Some(handle_clip.clone()); // Fixed cloning the handle
            }
            if blend_animation.second_anim == name.to_string() {
                blend_animation.second_anim_clip = Some(handle_clip.clone()); // Fixed cloning the handle
            }
        }

        // Masking according to given clips
        if let Some(anim_clip) = &blend_animation.first_anim_clip {
            if let Some(second_clip) = &blend_animation.second_anim_clip {
                let loaded_first_clip = assets_clips
                    .get(anim_clip)
                    .expect("The handle to grab the clip");
                let loaded_second_clip = assets_clips
                    .get(second_clip)
                    .expect("The handle to grab the clip");

                // Create a new animation clip
                let mut new_clip = AnimationClip::default();

                // Add curves for masked bones
                for target in masked_bones.iter() {
                    println!("{:?}", target.id);
                    if let Some(override_curves) = loaded_second_clip.curves_for_target(target.id) {
                        for curve in override_curves.iter() {
                            new_clip.add_curve_to_target(target.id, curve.clone());
                        }
                    }
                }

                // Add curves for not masked bones
                for other_target in not_masked.iter() {
                    println!("{:?}", other_target.id);
                    if let Some(current_curves) =
                        loaded_first_clip.curves_for_target(other_target.id)
                    {
                        for curve in current_curves.iter() {
                            new_clip.add_curve_to_target(other_target.id, curve.clone());
                        }
                    }
                }

                // Save blended clip
                let handle = assets_clips.add(new_clip);

                // Add the clip to the animation graph
                let node = blended_graph.add_clip(handle, 1.0, blended_graph.root.clone());

                new_nodes.push(node);
            }
        }
    }

    // Add graph to assets
    let asset_graph = assets_animation_graph.add(blended_graph);

    commands.insert_resource(BlendAnimations {
        animation_graph: asset_graph,
        node: new_nodes,
    });
}

// // Simple animation graph based on glt no fuss whatsoever just a bunch of nodes to be played
// pub fn spawn_animation_graph(
//     amount_players: Res<AmountPlayers>,
//     asset_pack: Res<MyAssets>,
//     assets_gltf: Res<Assets<Gltf>>,
//     mut assets_animation_graph: ResMut<Assets<AnimationGraph>>,
//     mut commands: Commands,
// ) {
//     for number_of_player in 1..=amount_players.quantity {
//         // Creating graphs according to amount of player
//         let mut graph = AnimationGraph::new();

//         // Node with a string name
//         let mut named_nodes = HashMap::new();

//         // Using bevy asset loader to easily access my assets
//         for (_, gltf_handle) in &asset_pack.gltf_files {
//             let gltf = assets_gltf
//                 .get(gltf_handle)
//                 .expect("My asset pack to have GLTF");

//             // Creating named nodes
//             for (name_animation, animation_clip) in gltf.named_animations.iter() {
//                 // Set the parent node depending on the animation name

//                 let node = graph.add_clip(animation_clip.clone(), 1.0, graph.root);

//                 // Creating named node
//                 named_nodes.insert(name_animation.to_string(), node);
//                 println!(
//                     "Current available animations are {} for player {}",
//                     name_animation, number_of_player
//                 );
//             }
//         }

//         // Adding animation graph to assets
//         let anim_graph = assets_animation_graph.add(graph);

//         // Formulating resource that tells me what is the name of the animation in a node and it is animation graph
//         commands.insert_resource(Animations {
//             named_nodes: named_nodes,
//             animation_graph: anim_graph.clone(),
//         });
//     }
// }
