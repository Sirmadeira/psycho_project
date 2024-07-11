use crate::player_effects::player_exists;
use crate::MyAppState;
use bevy::prelude::*;

pub struct TreatAnimations;

pub mod treat_animations;
pub mod  lib;

use self::{lib::*,treat_animations::*};

impl Plugin for TreatAnimations {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationType>();
        app.add_systems(
            Update,
            test_animations
                .run_if(player_exists)
                .run_if(in_state(MyAppState::InGame)),
        );
        app.add_systems(
            Update,
            state_machine
                .run_if(player_exists)
                .run_if(in_state(MyAppState::InGame)),
        );
    }
}