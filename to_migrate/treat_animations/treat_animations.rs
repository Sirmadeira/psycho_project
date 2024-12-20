use crate::treat_animations::lib::*;
use bevy::prelude::*;
use bevy::utils::Duration;

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

pub fn transition_animations(
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
            let animation = animations
                .named_nodes
                .get(&event.0.name)
                .expect("To find animation in resource");

            if let Some(active_anim) = animation_player.animation(current_animation) {
                if active_anim.elapsed() < 0.1 {
                    return;
                }
            }
            // Handles scenario where the is no "stun"
            if current_animation != *animation {
                if event.0.repeat {
                    println!("Playing animation repeat {}", event.0.name);
                    active_transitions
                        .play(&mut animation_player, *animation, event.0.duration)
                        .repeat();
                } else {
                    println!("Playing animation not repeat {}", event.0.name);
                    active_transitions.play(&mut animation_player, *animation, event.0.duration);
                }
            }
        }
    } else {
        println!("Adding first animation");
        let first_anim = animations
            .named_nodes
            .get("Idle")
            .expect("First animation to exist");
        active_transitions.play(&mut animation_player, *first_anim, Duration::ZERO);
    }
}
