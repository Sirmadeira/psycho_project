use bevy::prelude::*;

use self::{
    detect_hits::*, lib::*, move_character::*, rotate_character::*, status_effects::*,
    status_observers::*,
};

pub mod detect_hits;
pub mod lib;
pub mod move_character;
pub mod rotate_character;
pub mod status_effects;
pub mod status_observers;

use crate::{spawn_game_entities::player_exists, MyAppState};

pub struct PlayerEffects;

impl Plugin for PlayerEffects {
    fn build(&self, app: &mut App) {
        app.add_event::<TypeOfAttack>();
        app.add_event::<MovementAction>();
        app.add_event::<RotateAction>();
        app.register_type::<StatusEffectDash>();
        app.register_type::<StatusEffectWallBounce>();
        app.register_type::<StatusEffectStun>();
        // Gives status effects
        app.add_systems(
            Update,
            (
                check_status_grounded,
                check_status_effect,
                check_status_wallbounce,
                check_status_idle,
                check_dead,
            )
                .run_if(player_exists)
                .run_if(in_state(MyAppState::InGame)),
        );
        app.observe(observe_grounded);
        // Send animation events and at the same time, movement events
        app.add_systems(
            Update,
            (keyboard_walk, keyboard_dash, keyboard_jump)
                .run_if(player_exists)
                .run_if(in_state(MyAppState::InGame)),
        );
        // Moves character around
        app.add_systems(
            FixedUpdate,
            (
                move_character,
                rotate_character,
                detect_rotation,
                spine_look_at,
            )
                .run_if(player_exists)
                .run_if(in_state(MyAppState::InGame)),
        );

        app.add_systems(
            FixedUpdate,
            (
                detect_hits_body_weapon,
                detect_hits_wall_weapon,
                detect_hits_weapon_weapon,
            )
                .run_if(player_exists)
                .run_if(in_state(MyAppState::InGame)),
        );
    }
}
