use bevy::prelude::*;

use crate::load_assets_plugin::MyAssets;
use crate::spawn_game_entities::lib::{AmountPlayers,Player};
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
    animations: Res<Animations>,
    mut commands: Commands,
    mut players: Query<Entity, Added<AnimationPlayer>>,
) {
    // Each skinned mesh already  comes with a prespawned animation player struct
    for entity in &mut players {
        commands
            .entity(entity)
            .insert(animations.animation_graph.clone());
    }
}

// Adds necessary components
pub fn setup_state_machine(
    animations: Res<Animations>,
    mut animated_entities: Query<(Entity,&mut AnimationPlayer),Added<AnimationPlayer>>,
    mut commands: Commands,
) {

    for (entity,mut animation_player) in  animated_entities.iter_mut(){
        let mut transitions = AnimationTransitions::new();
        transitions.play(
            &mut animation_player,
            animations.named_nodes["Idle"],
            Duration::ZERO,
        );
        commands.entity(entity).insert(transitions);
    }
}

// This will handle animation according to input events given by player_effects or other plugins

pub fn state_machine(
    player_skeleton: Query<(Entity, Has<AnimationCooldown>, Has<DiagonalAnimation>), With<Player>>,
    mut animation_components: Query<(&mut AnimationPlayer,&mut AnimationTransitions),With<AnimatedEntity>>,
    animations: Res<Animations>,
    mut after_animation: EventWriter<AfterAnim>,
    mut animation_to_play: EventReader<AnimationType>,
    mut commands: Commands,
) {
    // Ensuring that this solely affects player
    let (player_entity, has_cooldown, has_diagonal) = player_skeleton
        .get_single()
        .expect("Expected to have exactly one player entity");

    let (mut animation_player, mut active_transitions) = animation_components
        .get_single_mut()
        .expect("Expect to have animated armature");

    let current_animation = active_transitions
        .get_main_animation()
        .expect("Expected to always have an active transition");

    for event in animation_to_play.read() {
        let properties = event.properties();

        let animation = animations
            .named_nodes
            .get(properties.name)
            .expect("To find animation in resource");

        // Handles scenario where the is no after anim
        if current_animation != *animation && properties.after_anim.is_none() {
            if has_cooldown {
                continue;
            } else if properties.repeat {
                active_transitions
                    .play(&mut animation_player, *animation, properties.duration)
                    .repeat();
            } else {
                active_transitions.play(&mut animation_player, *animation, properties.duration);
            }
            commands.entity(player_entity).remove::<DiagonalAnimation>();
        }
        // Handle first scenario where he just entered an animation that has after anim
        else if properties.after_anim.is_some() && !has_diagonal {
            active_transitions.play(&mut animation_player, *animation, properties.duration);
            commands.entity(player_entity).insert(DiagonalAnimation);
        }
        // Emits event when active animation is_finished later to seek to
        else if properties.after_anim.is_some() && has_diagonal {
            let active_animation = animation_player
                .animation_mut(current_animation)
                .expect("To be playing animation");
            // Handle scenario where animation become repetable
            if active_animation.is_finished() {
                after_animation.send(AfterAnim(&properties.after_anim.unwrap()));
            }
        }
        // Adds cooldown if there is
        if let Some(cooldown) = properties.cooldown {
            commands
                .entity(player_entity)
                .insert(AnimationCooldown(Timer::new(cooldown, TimerMode::Once)));
        }
    }
}

pub fn after_anim_state_machine(
    mut animation_components: Query<(&mut AnimationPlayer,&mut AnimationTransitions),With<AnimatedEntity>>,
    mut after_animation: EventReader<AfterAnim>,
    animations: Res<Animations>,
) {


    let ( mut animation_player,mut active_transition) = animation_components
        .get_single_mut()
        .expect("To exist for only one main player");

    for animation in after_animation.read() {
        let properties = animation.properties();
        let animation = animations
            .named_nodes
            .get(properties.name)
            .expect("For after anim to have correct name");
        active_transition
            .play(&mut animation_player, *animation, properties.duration)
            .repeat();
    }
}
