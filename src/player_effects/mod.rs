use bevy::prelude::*;

use self::{detect_hits::*, lib::*, move_character::*, status_effects::*};

pub mod detect_hits;
pub mod lib;
pub mod move_character;
pub mod status_effects;

use crate::spawn_game_entities::player_exists;

pub struct PlayerEffects;

impl Plugin for PlayerEffects {
    fn build(&self, app: &mut App) {
        app.add_event::<TypeOfAttack>();
        app.add_event::<MovementAction>();
        app.register_type::<StatusEffectDash>();
        app.register_type::<StatusEffectWallBounce>();
        app.add_systems(
            Update,
            (
                check_status_grounded,
                check_status_effect,
                check_status_wallbounce,
                check_status_idle,
                check_dead,
            )
                .run_if(player_exists),
        );
        app.add_systems(
            Update,
            (keyboard_walk, keyboard_dash, keyboard_jump).run_if(player_exists)
        );
        // Side physics
        app.add_systems(
            FixedUpdate,
            (move_character, head_look_at).run_if(player_exists),
        );
        app.add_systems(FixedUpdate,(detect_hits_body_weapon,detect_hits_wall_weapon,detect_hits_weapon_weapon).run_if(player_exists));
        
    }
}

