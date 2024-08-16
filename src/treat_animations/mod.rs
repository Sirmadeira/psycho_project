use bevy::prelude::*;
use crate::MyAppState;


pub struct TreatAnimations;

pub mod lib;
pub mod treat_animations;
pub mod setup_entities;

use self::{setup_entities::*,lib::*, treat_animations::*};

use crate::spawn_game_entities::lib::StateSpawnScene;

use crate::form_player::*;

impl Plugin for TreatAnimations {
    fn build(&self, app: &mut App) {
        // Configs
        app.add_event::<AnimationType>();
        app.insert_resource(ConfigBoneMaskedAnimations::default());
        // Animation debug
        app.register_type::<Animations>();
        // app.add_systems(OnEnter(MyAppState::InGame), spawn_animation_graph);
        app.add_systems(
            OnEnter(StateSpawnScene::Done),
            (mark_bones, blend_animations).chain(),
        );
        app.add_systems(
            Update,
            (add_animation_graph, setup_state_machine)
                .run_if(player_exists)
                .run_if(in_state(MyAppState::InGame)),
        );
        app.add_systems(
            Update,
            state_machine
                .run_if(player_exists)
                .run_if(in_state(MyAppState::InGame))
                .after(setup_state_machine),
        );
    }
}
