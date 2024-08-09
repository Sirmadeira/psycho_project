use std::time::Duration;

use crate::player_effects::lib::StatusEffectStun;
use crate::spawn_game_entities::lib::*;
use crate::treat_animations::lib::*;
use bevy::prelude::*;

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
    animated_entities: Query<Entity, Added<AnimationPlayer>>,
    mut commands: Commands,
) {
    for entity in animated_entities.iter() {
        let transitions = AnimationTransitions::new();
        commands.entity(entity).insert(transitions);
    }
}

// This will handle animation according to input events given by player_effects or other plugins

pub fn state_machine(
    mut stun_info: Query<&mut StatusEffectStun, With<Player>>,
    mut animation_components: Query<
        (&mut AnimationPlayer, &mut AnimationTransitions),
        With<AnimatedEntity>,
    >,
    animations: Res<Animations>,
    mut animation_to_play: EventReader<AnimationType>,
) {
    let (mut animation_player, mut active_transitions) = animation_components
        .get_single_mut()
        .expect("Expect to have animated armature");

    if let Some(current_animation) = active_transitions.get_main_animation() {
        for event in animation_to_play.read() {
            let properties = event.properties();

            let animation = animations
                .named_nodes
                .get(properties.name)
                .expect("To find animation in resource");

            if let Ok(mut stun) = stun_info.get_single_mut() {
                println!("Animation to play {}", properties.name);
                if !stun.played_animation {
                    active_transitions.play(&mut animation_player, *animation, properties.duration);
                    stun.played_animation = true;
                }
                return;
            } else {
                println!("Animation to play {}", properties.name);
                // Handles scenario where the is no "stun"
                if current_animation != *animation {
                    if properties.repeat {
                        active_transitions
                            .play(&mut animation_player, *animation, properties.duration)
                            .repeat();
                    } else {
                        active_transitions.play(
                            &mut animation_player,
                            *animation,
                            properties.duration,
                        );
                    }
                }
            }
        }
    } else {
        println!("Adding first animation");
        let first_anim = animations
            .named_nodes
            .get("StartPose")
            .expect("First animation to exist");
        active_transitions.play(&mut animation_player, *first_anim, Duration::ZERO);
    }
}
