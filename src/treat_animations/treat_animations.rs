use crate::spawn_game_entities::lib::*;
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
    animations: Res<Animations>,
    mut animated_entities: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    mut commands: Commands,
) {
    for (entity, mut animation_player) in animated_entities.iter_mut() {
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
    player_skeleton: Query<(Entity, Has<AnimationCooldown>), With<Player>>,
    mut animation_components: Query<
        (&mut AnimationPlayer, &mut AnimationTransitions),
        With<AnimatedEntity>,
    >,
    animations: Res<Animations>,
    mut animation_to_play: EventReader<AnimationType>,
    mut commands: Commands,
) {
    // Ensuring that this solely affects player
    let (player_entity, has_cooldown) = player_skeleton
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
        if current_animation != *animation {
            println!("Animation to play {}",properties.name);
            if has_cooldown {
                continue;
            } else if properties.repeat {
                active_transitions
                    .play(&mut animation_player, *animation, properties.duration)
                    .repeat();
            } else {
                active_transitions.play(&mut animation_player, *animation, properties.duration);
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
