use bevy::prelude::*;
use crate::player_effects::lib::Grounded;
use crate::spawn_game_entities::lib::Player;
use crate::treat_animations::lib::AnimationType;


pub fn observe_grounded(
    trigger: Trigger<OnInsert, Grounded>,
    q_1: Query<&Player>,
    mut animation_writer: EventWriter<AnimationType>,
) {
    // Most probably will be player or side player
    if q_1.get(trigger.entity()).is_ok() {
        animation_writer.send(AnimationType::Landing);
    } else {
        println!("Side player doesn't count");
    }
}