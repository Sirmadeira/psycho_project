use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use self::{lib::*, move_character::*, spawn_objects::*, status_effects::*,health_mechanics::*};

pub mod lib;
pub mod spawn_objects;
pub mod status_effects;
pub mod health_mechanics;
pub mod move_character;

use crate::mod_char_plugin::lib::StateSpawnScene;

pub struct PlayerEffectsPlugin;

impl Plugin for PlayerEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PdInfo>();
        app.register_type::<Timers>();
        app.register_type::<Limit>();
        app.register_type::<Health>();
        app.register_type::<StatusEffectDash>();
        app.add_event::<MovementAction>();
        app.add_systems(OnEnter(StateSpawnScene::Done), spawn_main_rigidbody);
        app.init_state::<StatePlayerCreation>();
        app.add_systems(
            Update,
            (
                // Check status effects on player
                check_status_grounded,
                check_status_effect,
                // Input handler
                keyboard_walk,
                keyboard_dash,
                keyboard_jump,
                // Event manager
            )
                .chain()
                .run_if(in_state(StatePlayerCreation::Done)),
        );
        app.add_systems(FixedUpdate, display_events);
        app.add_systems(
            FixedUpdate,
            move_character.run_if(in_state(StatePlayerCreation::Done)),
        );
        app.add_systems(
            FixedUpdate,
            player_look_at_camera.run_if(in_state(StatePlayerCreation::Done)),
        );
        app.add_systems(
            FixedUpdate,
            detect_hits.run_if(in_state(StatePlayerCreation::Done)),
        );
    }
}

// Display collision events between colliders
fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.read() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.read() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}
