use std::ops::Add;

use bevy::prelude::*;

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
    player_skeleton: Query<(Entity, Has<AnimationCooldown>,Has<DiagonalAnimation>), With<Player>>,
    mut components: Query<(&mut AnimationPlayer, Option<&mut AnimationTransitions>)>,
    children_entities: Query<&Children>,
    names: Query<&Name>,
    animations: Res<Animations>,
    mut after_animation: EventWriter<AfterAnim>,
    mut animation_to_play: EventReader<AnimationType>,
    mut commands: Commands,
) {
    // Ensuring that this is the player animation
    let (player_entity, has_cooldown,has_diagonal) = player_skeleton
        .get_single()
        .expect("Expected to have exactly one player entity");

    let animated_entity =
        find_child_with_name_containing(&children_entities, &names, &player_entity, "Armature")
            .expect("Expected to find an Armature child");

    let (mut animation_player, active_transitions) = components
        .get_mut(animated_entity)
        .expect("Expected to find skeleton components");

    if let Some(mut active_transition) = active_transitions {
        let current_animation = active_transition
            .get_main_animation()
            .expect("Expected to always have an active transition");

        for event in animation_to_play.read() {
            // Just a bunch of info
            let properties = event.properties();

            // Adds animation if there is
            if let Some(animation) = animations.named_nodes.get(properties.name) {
                
                if current_animation != *animation && properties.after_anim.is_none() {
                    commands.entity(player_entity).remove::<DiagonalAnimation>();
                    // If has cooldown can transition
                    if has_cooldown {
                        continue;
                    } else if properties.repeat {
                        active_transition
                            .play(&mut animation_player, *animation, properties.duration)
                            .repeat();
                    } else {
                        active_transition.play(
                            &mut animation_player,
                            *animation,
                            properties.duration,
                        );
                    }
                }
                // Handle first scenario where he just entered an animation that has after anim
                else if properties.after_anim.is_some() && !has_diagonal{
                    active_transition.play(&mut animation_player, *animation, properties.duration);
                    commands.entity(player_entity).insert(DiagonalAnimation);
                }
                // Emits event when active animation is_finished later to seek to
                else if properties.after_anim.is_some() && has_diagonal{
                    let active_animation = animation_player.animation_mut(current_animation).expect("To be playing animation");
                    if active_animation.is_finished(){
                        after_animation.send(AfterAnim(&properties.after_anim.unwrap()));
                    }
                }

            }
            // Adds cooldown if there is
            if let Some(cooldown) = properties.cooldown {
                commands
                    .entity(player_entity)
                    .insert(AnimationCooldown(Timer::new(cooldown, TimerMode::Once)));
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


pub fn after_anim_state_machine(mut animation_components: Query<(&mut AnimationTransitions,&mut AnimationPlayer),With<AnimationTransitions>>,   
         mut after_animation: EventReader<AfterAnim>,
         animations: Res<Animations>,){


    let (mut active_transition,mut animation_player) = animation_components.get_single_mut().expect("To exist for no only one main player");

    for animation in after_animation.read(){
        let properties = animation.properties();
        let animation = animations.named_nodes.get(properties.name).expect("For after anim to have correct name");
        active_transition.play(&mut animation_player, *animation, properties.duration).repeat();

    }   

}