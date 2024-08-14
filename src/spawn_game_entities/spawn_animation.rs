use bevy::prelude::*;

use bevy::utils::HashMap;

use crate::load_assets_plugin::MyAssets;
use crate::spawn_game_entities::lib::*;

pub fn spawn_animation_graph(
    amount_players: Res<AmountPlayers>,
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut assets_animation_graph: ResMut<Assets<AnimationGraph>>,
    mut commands: Commands,
) {
    for number_of_player in 1..=amount_players.quantity {
        // Creating graphs according to amount of player
        let mut graph = AnimationGraph::new();

        // Node with a string name
        let mut named_nodes = HashMap::new();

        let blend_node = graph.add_blend(0.5, graph.root);

        // Using bevy asset loader to easily access my assets
        for (_, gltf_handle) in &asset_pack.gltf_files {
            let gltf = assets_gltf
                .get(gltf_handle)
                .expect("My asset pack to have GLTF");

            // Creating named nodes
            for (name_animation, animation_clip) in gltf.named_animations.iter() {
                // Set the parent node depending on the animation name
                let parent_node = if name_animation.as_ref() == "Idle"
                    || name_animation.as_ref() == "FrontWalk"
                {
                    println!("{}", name_animation);
                    blend_node
                } else {
                    graph.root
                };

                let node = graph.add_clip(animation_clip.clone(), 1.0, parent_node);

                // Creating named node
                named_nodes.insert(name_animation.to_string(), node);
                println!(
                    "Current available animations are {} for player {}",
                    name_animation, number_of_player
                );
            }
        }

        // Adding animation graph to assets
        let anim_graph = assets_animation_graph.add(graph);

        // Formulating resource that tells me what is the name of the animation in a node and it is animation graph
        commands.insert_resource(Animations {
            named_nodes: named_nodes,
            animation_graph: anim_graph.clone(),
        });
    }
}
