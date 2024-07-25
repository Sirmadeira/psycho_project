use crate::player_effects::player_exists;
use crate::MyAppState;
use bevy::prelude::*;

pub struct TreatAnimations;

pub mod lib;
pub mod treat_animations;

use self::{lib::*, treat_animations::*};

impl Plugin for TreatAnimations {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationType>();
        app.add_event::<AfterAnim>();
        app.register_type::<Animations>();
        app.add_systems(OnEnter(MyAppState::InGame), spawn_animations_graphs);
        //Weird i know but u need to do this all the time
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
    }
}
