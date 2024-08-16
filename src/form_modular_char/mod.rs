use crate::MyAppState;
use bevy::prelude::*;

pub mod helpers;
pub mod lib;
pub mod setup_entities;

use self::{lib::*, setup_entities::*};

pub struct FormModularChar;

impl Plugin for FormModularChar {
    fn build(&self, app: &mut App) {
        // Creating modular character
        app.add_systems(
            OnEnter(MyAppState::CreatingCharacter),
            spawn_skeleton_and_attachments.chain(),
        );
        app.add_systems(
            OnEnter(MyAppState::TranferingAnimations),
            (
                transfer_animation,
                make_end_entity,
                disable_culling_for_skinned_meshes,
            ),
        );
        // Amount of player configuration - Tells me how many to spawn
        app.insert_resource(AmountPlayers { quantity: 2 });
        // Tell me what visual and weapons to attack
        app.insert_resource(ConfigModularCharacters {
            visuals_to_be_attached: vec![String::from("rigge_female")],
            weapons_to_be_attached: vec![String::from("katana")],
        });
    }
}
