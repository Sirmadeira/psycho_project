use bevy::prelude::*;

pub mod setup_entities;
pub mod lib;

use crate::MyAppState;

use self::{setup_entities::*,lib::*};
pub struct FormPlayer;

impl Plugin for FormPlayer {
    fn build(&self, app: &mut App) {
        // Spawn player debug
        app.register_type::<PlayerVel>();
        app.register_type::<DashTimers>();
        app.register_type::<Limit>();
        app.register_type::<Health>();
        app.register_type::<StateOfAttack>();
        // Create player
        app.add_systems(OnEnter(MyAppState::CharacterCreated), spawn_main_rigidbody);
    }
}
