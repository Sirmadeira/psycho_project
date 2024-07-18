use bevy::prelude::*;
use crate::MyAppState;
use crate:: player_effects::player_exists;

pub struct TreatAnimations;

pub mod treat_animations;
pub mod  lib;

use self::{lib::*,treat_animations::*};

impl Plugin for TreatAnimations {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationType>();
        app.register_type::<Animations>();
        app.add_systems(OnEnter(MyAppState::InGame), spawn_animations_graphs);
        app.add_systems(
            OnEnter(MyAppState::InGame),
            add_animation_graph
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