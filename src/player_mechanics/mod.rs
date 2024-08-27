// Player mechanics will be solely here- Movement rotatinon detect it is hits and so on.

use bevy::prelude::*;

use self::{detect_system::*, keyboard_system::*, lib::*, move_rotate::*, tick_status::*};

pub mod detect_system;
pub mod keyboard_system;
pub mod lib;
pub mod move_rotate;
pub mod tick_status;

use crate::MyAppState;

use crate::form_player::*;

pub struct PlayerMechanics;

impl Plugin for PlayerMechanics {
    fn build(&self, app: &mut App) {
        // Events
        app.add_event::<MovementAction>();
        app.add_event::<PlayerAction>();
        // Debugin
        app.register_type::<StatusEffectDash>();
        app.register_type::<StatusEffectStun>();

        // Detect systems - They run in fixed preupdate because they define game logic. For example if idle, send idle animation and if ground dont fly, so on on
        app.add_systems(
            FixedPreUpdate,
            (
                detect_hits_body_weapon,
                detect_hits_wall_weapon,
                detect_hits_weapon_weapon,
                detect_hits_body_ground,
                detect_dead,
            )
                .run_if(player_exists)
                .run_if(in_state(MyAppState::InGame)),
        );

        // Ticker related systems - They just remove components it would be ideal to put them in post, because them new status can be applied and evaluated correctly.
        // Since they are a lot of timers runs in fixed to avoid fps related issues and so on
        app.add_systems(
            FixedPostUpdate,
            (check_status_ticker,)
                .run_if(player_exists)
                .run_if(in_state(MyAppState::InGame)),
        );

        // Send movement events and anImation events
        app.add_systems(
            Update,
            (keyboard_walk, keyboard_dash, keyboard_jump, keyboard_attack)
                .run_if(player_exists)
                .run_if(in_state(MyAppState::InGame)),
        );

        // Moves character around - Runs in update- Because they just dont care about status
        app.add_systems(
            Update,
            (move_character, rotate_character)
                .run_if(player_exists)
                .run_if(in_state(MyAppState::InGame)),
        );
        app.add_systems(
            Update,
            player_state
                .run_if(player_exists)
                .run_if(in_state(MyAppState::InGame))
                .after(move_character)
                .after(keyboard_attack),
        );

        // Just an aditional visual mechanic - Doesnt really matter as long as it happens before camera sync player camera.
        app.add_systems(
            Update,
            head_look_at
                .run_if(player_exists)
                .run_if(in_state(MyAppState::InGame)),
        );
    }
}
