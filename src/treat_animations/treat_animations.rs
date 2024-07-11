use bevy::prelude::*;

use bevy::utils::Duration;
use crate::mod_char_plugin::lib::Animations;

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
                animations.named_nodes["RightAttack"],
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