use bevy::prelude::*;

pub mod setup_entities;

use crate::form_modular_char::lib::StateSpawnScene;

use self::setup_entities::*;
pub struct FormPlayer;

impl Plugin for FormPlayer {
    fn build(&self, app: &mut App) {
        // Spawn player debug
        app.register_type::<PdInfo>();
        app.register_type::<Timers>();
        app.register_type::<Limit>();
        app.register_type::<Health>();
        // Create player
        app.add_systems(OnEnter(StateSpawnScene::Done), spawn_main_rigidbody);
    }
}

// Player condition
pub fn player_exists(player_q: Query<Entity, With<Player>>) -> bool {
    match player_q.get_single() {
        Ok(_) => true,
        Err(_) => false,
    }
}