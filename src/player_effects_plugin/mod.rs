use bevy::prelude::*;

use self::{detect_hits::*, lib::*, move_character::*, spawn_objects::*, status_effects::*};

use crate::mod_char_plugin::all_chars_created;

pub mod detect_hits;
pub mod lib;
pub mod move_character;
pub mod spawn_objects;
pub mod status_effects;

use crate::mod_char_plugin::lib::StateSpawnScene;
use crate:: MyPlayerSet;

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
        app.add_systems(OnEnter(StateSpawnScene::Done), spawn_main_rigidbody.in_set(MyPlayerSet::SpawnEntities));
        app.add_systems(
            Update,
            (
                check_status_grounded,
                check_status_effect,
                check_status_wallbounce,
                check_dead,
            )
                .in_set(MyPlayerSet::HandleStatusEffects)
        );
        app.add_systems(
            Update,
            (
                keyboard_walk,
                keyboard_dash,
                keyboard_jump,
                keyboard_attack,
            )
                .in_set(MyPlayerSet::HandleInputs)
        );
        // Side physics
        app.add_systems(
            FixedUpdate,
            (move_character, player_look_at_camera)
                .in_set(MyPlayerSet::SidePhysics)
        );
        // Detecting specific hits
        app.add_systems(
            FixedUpdate,
            (
                detect_hits_body_weapon,
                detect_hits_wall_weapon,
                detect_hits_weapon_weapon,
            )
                .in_set(MyPlayerSet::DetectCollisions)
        );
        app.configure_sets(OnEnter(StateSpawnScene::Done), MyPlayerSet::SpawnEntities.run_if(all_chars_created));
        app.configure_sets(Update, (MyPlayerSet::HandleInputs,MyPlayerSet::HandleInputs).run_if(player_exists));
        app.configure_sets(FixedUpdate, (MyPlayerSet::SidePhysics,MyPlayerSet::DetectCollisions).run_if(player_exists));
        
    }
}

pub fn player_exists(player_q: Query<Entity, With<Player>>) -> bool {
    match player_q.get_single() {
        Ok(_) => {
            true
        },
        Err(_) => false,
    }
}