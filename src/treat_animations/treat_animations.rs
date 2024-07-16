
use bevy::prelude::*;

use bevy::utils::{Duration,HashMap};
use crate::treat_animations::lib::*;
use crate::load_assets_plugin::MyAssets;
use crate::mod_char::lib::AmountPlayers;
// Creates animation graph for each player and add it is clips to it
pub fn spawn_animations_graphs(
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

        // Using bevy asset loader to easily acess my assets
        for (_, gltf_handle) in &asset_pack.gltf_files {
            let gltf = assets_gltf
                .get(gltf_handle)
                .expect("My asset pack to have GLTF");

            // Creating named nodes
            for (name_animation, animation_clip) in gltf.named_animations.iter() {
                // Returns animations node
                let node = graph.add_clip(animation_clip.clone(), 1.0, graph.root);
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


pub fn test_animations(
    mut commands: Commands,
    animations: Res<Animations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    // Each skinned mesh already  comes with a prespawned animation player struct
    for (entity, mut player) in &mut players {
        let mut transitions = AnimationTransitions::new();
        transitions
            .play(
                &mut player,
                animations.named_nodes["TPose"],
                Duration::ZERO,
            )
            .repeat();
        // Display the transitions of the current entity
        commands
            .entity(entity)
            .insert(animations.animation_graph.clone())
            .insert(transitions);
    }
}


pub fn state_machine() {}