use crate::MyAppState;
use bevy::prelude::*;

pub struct TreatAnimations;

pub mod lib;
pub mod setup_entities;
pub mod treat_animations;

use self::{lib::*, setup_entities::*, treat_animations::*};

use crate::form_player::*;

impl Plugin for TreatAnimations {
    fn build(&self, app: &mut App) {
        // Configs
        app.add_event::<AnimationType>();
        app.insert_resource(ConfigBoneMaskedAnimations::default());
        // Animation debug
        app.register_type::<Animations>();
        app.add_systems(
            OnEnter(MyAppState::CharacterCreated),
            (mark_bones, create_blend_animations, gltf_animations).chain(),
        );
        app.add_systems(
            Update,
            (add_animation_graph, setup_state_machine)
                .run_if(player_exists)
                .run_if(in_state(MyAppState::CharacterCreated)),
        );
        app.add_systems(
            Update,
            state_machine
                .run_if(player_exists)
                .run_if(in_state(MyAppState::InGame)),
        );
    }
}
