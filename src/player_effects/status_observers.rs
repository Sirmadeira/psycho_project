use bevy::prelude::*;
use crate::player_effects::lib::{Grounded,StatusEffectStun};
use crate::spawn_game_entities::lib::Player;
use crate::treat_animations::lib::AnimationType;



pub fn observe_grounded(
    trigger: Trigger<OnInsert, Grounded>,
    q_1: Query<&Player>,
    mut animation_writer: EventWriter<AnimationType>,
    mut commands: Commands
) {
    let character_grounded  = trigger.entity();
    // Check if player
    if q_1.get(character_grounded).is_ok() {
        println!("Stun");
        animation_writer.send(AnimationType::Landing);
        let animation_cd = AnimationType::Landing.properties().cooldown.expect("This animation to have a cooldown");
        commands.entity(character_grounded).insert(StatusEffectStun{timer: Timer::new(animation_cd, TimerMode::Once),played_animation: false});
        
    } else {
        println!("Side player doesn't count");
    }
}