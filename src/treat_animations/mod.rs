use crate::spawn_game_entities::player_exists;
use crate::MyAppState;
use bevy::prelude::*;

pub struct TreatAnimations;

pub mod lib;
pub mod treat_animations;

use self::{lib::*, treat_animations::*};

impl Plugin for TreatAnimations {
    fn build(&self, app: &mut App) {
        app.add_event::<AfterAnim>();
        app.add_event::<AnimationType>();
        app.add_systems(
            Update,
            (add_animation_graph, setup_state_machine)
                .run_if(player_exists)
                .run_if(in_state(MyAppState::InGame)),
        );
        app.add_systems(
            Update,
            (state_machine, after_anim_state_machine)
                .run_if(player_exists)
                .run_if(in_state(MyAppState::InGame))
                .after(setup_state_machine)
                .chain(),
        );
        app.add_systems(
            Update,
            apply_diagonal
                .run_if(player_exists)
                .run_if(in_state(MyAppState::InGame))
                .after(setup_state_machine)
        );
    }
}
