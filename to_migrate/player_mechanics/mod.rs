// Player mechanics will be solely here- Movement rotatinon detect it is hits and so on.

use bevy::prelude::*;

use self::{detect_system::*, keyboard_system::*, lib::*, move_rotate::*};

pub mod detect_system;
pub mod keyboard_system;
pub mod lib;
pub mod move_rotate;

use crate::MyAppState;

pub struct PlayerMechanics;

impl Plugin for PlayerMechanics {
    fn build(&self, app: &mut App) {
        // Events
        app.add_event::<MovementAction>();
        app.add_event::<PlayerAction>();
        // Debugin
        app.register_type::<StatusEffectDash>();
        app.register_type::<StatusEffectStun>();

        app.configure_sets(
            PreUpdate,
            PlayerSystems::DetectCollisions.run_if(in_state(MyAppState::InGame)),
        );

        app.configure_sets(
            PreUpdate,
            PlayerSystems::KeyboardInput
                .run_if(in_state(MyAppState::InGame))
                .after(PlayerSystems::DetectCollisions),
        );

        app.configure_sets(
            Update,
            PlayerSystems::MovePlayer
                .run_if(in_state(MyAppState::InGame))
                .after(PlayerSystems::KeyboardInput),
        );

        app.configure_sets(
            Update,
            PlayerSystems::StatePlayer
                .run_if(in_state(MyAppState::InGame))
                .after(PlayerSystems::KeyboardInput)
                .after(PlayerSystems::MovePlayer),
        );

        app.add_systems(
            PreUpdate,
            (
                detect_hits_body_weapon,
                detect_hits_wall_weapon,
                detect_hits_weapon_weapon,
                detect_hits_body_ground,
                detect_dead,
            )
                .in_set(PlayerSystems::DetectCollisions),
        );

        app.add_systems(
            PreUpdate,
            (keyboard_walk, keyboard_dash, keyboard_jump, keyboard_attack)
                .in_set(PlayerSystems::KeyboardInput),
        );

        app.add_systems(
            Update,
            (move_character, rotate_character).in_set(PlayerSystems::MovePlayer),
        );

        app.add_systems(
            Update,
            player_state_to_animation.in_set(PlayerSystems::StatePlayer),
        );

        app.add_systems(Update, head_look_at.run_if(in_state(MyAppState::InGame)));
    }
}
