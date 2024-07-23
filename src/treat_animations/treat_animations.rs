use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::load_assets_plugin::MyAssets;
use crate::mod_char::helpers::find_child_with_name_containing;
use crate::mod_char::lib::AmountPlayers;
use crate::player_effects::lib::Player;
use crate::treat_animations::lib::*;
use bevy::utils::{Duration, HashMap};

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

// Loads from assets and put into our animations players must have for animation playing
pub fn add_animation_graph(
    mut commands: Commands,
    animations: Res<Animations>,
    mut players: Query<Entity, Added<AnimationPlayer>>,
) {
    // Each skinned mesh already  comes with a prespawned animation player struct
    for entity in &mut players {
        commands
            .entity(entity)
            .insert(animations.animation_graph.clone());
    }
}

// This will handle animation according to input events given by player_effects or other plugins

pub fn state_machine(
    player_skeleton: Query<Entity, With<Player>>,
    mut components: Query<(&mut AnimationPlayer, Option<&mut AnimationTransitions>)>,
    children_entities: Query<&Children>,
    names: Query<&Name>,
    animations: Res<Animations>,
    mut animation_to_play: EventReader<AnimationType>,
    mut commands: Commands,
) {
    // Ensuring that this is the player animation
    let player_entity = player_skeleton
        .get_single()
        .expect("Expected to have exactly one player entity");

    let animated_entity = find_child_with_name_containing(&children_entities, &names, &player_entity, "Armature")
        .expect("Expected to find an Armature child");

    let (mut animation_player, active_transitions) = components
        .get_mut(animated_entity)
        .expect("Expected to find skeleton components");

    if let Some(mut active_transition) = active_transitions {
        let current_animation = active_transition
            .get_main_animation()
            .expect("Expected to always have an active transition");

        for event in animation_to_play.read() {
            let (animation_name, duration, repeat) = match event {
                AnimationType::Idle => ("Idle", Duration::from_millis(200), false),
                AnimationType::FrontWalk => ("FrontWalk", Duration::from_millis(200), true),
                AnimationType::BackWalk => ("BackWalk", Duration::from_millis(200), true),
                AnimationType::LeftWalk => ("LeftWalk", Duration::from_millis(200), true),
                AnimationType::RightWalk => ("RightWalk", Duration::from_millis(200), true),
                AnimationType::RightDigWalk => ("RightDigWalk", Duration::from_millis(200), false),
                AnimationType::BackRightDigWalk => ("BackRightDigWalk", Duration::from_millis(200), false),
                AnimationType::LeftDigWalk => ("LeftDigWalk", Duration::from_millis(200), false),
                AnimationType::BackLeftDigWalk => ("BackLeftDigWalk", Duration::from_millis(200), false),
                AnimationType::None => continue, // Skip if no animation
            };

            let animation = &animations.named_nodes[animation_name];
            if current_animation != *animation {
                println!("That animation was different: {}", animation_name);
                if repeat {
                    active_transition
                        .play(&mut animation_player, *animation, duration)
                        .repeat();
                } else {
                    active_transition.play(&mut animation_player, *animation, duration);
                }
            }
        }
    } else {
        let mut transitions = AnimationTransitions::new();
        transitions.play(
            &mut animation_player,
            animations.named_nodes["Idle"],
            Duration::ZERO,
        );
        commands.entity(animated_entity).insert(transitions);
    }
}

