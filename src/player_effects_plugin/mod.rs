use bevy::prelude::*;

use self::{detect_hits::*, lib::*, move_character::*, spawn_objects::*, status_effects::*};

pub mod detect_hits;
pub mod lib;
pub mod move_character;
pub mod spawn_objects;
pub mod status_effects;

use crate::mod_char_plugin::lib::StateSpawnScene;

pub struct PlayerEffectsPlugin;

impl Plugin for PlayerEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PdInfo>();
        app.register_type::<Timers>();
        app.register_type::<Limit>();
        app.register_type::<Health>();
        app.register_type::<StatusEffectDash>();
        app.register_type::<StatusEffectWallBounce>();
        app.add_event::<MovementAction>();
        app.add_event::<TypeOfAttack>();
        app.add_systems(OnEnter(StateSpawnScene::Done), spawn_main_rigidbody);
        app.init_state::<StatePlayerCreation>();
        app.add_systems(
            Update,
            (
                // Check status effects on player
                check_status_grounded,
                check_status_effect,
                check_status_wallbounce,
                check_dead,
                // Input handler
                keyboard_walk,
                keyboard_dash,
                keyboard_jump,
                keyboard_attack,
                // Event manager
            )
                .run_if(in_state(StatePlayerCreation::Done)),
        );
        app.add_systems(
            FixedUpdate,
            move_character.run_if(in_state(StatePlayerCreation::Done)),
        );
        app.add_systems(
            FixedUpdate,
            player_look_at_camera.run_if(in_state(StatePlayerCreation::Done)),
        );
        // Detecting specific hits
        app.add_systems(
            FixedUpdate,
            (detect_hits_body_weapon,detect_hits_wall_weapon,detect_hits_weapon_weapon).run_if(in_state(StatePlayerCreation::Done)),
        );
    }
}
