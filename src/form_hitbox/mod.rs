use bevy::prelude::*;
use bevy::transform::TransformSystem;

pub mod setup_entities;
mod follow_along;
mod helpers;

use self::{setup_entities::*,follow_along::*};
use crate::spawn_game_entities::all_chars_created;
use crate::spawn_game_entities::lib::StateSpawnScene;


pub struct FormHitbox;

impl Plugin for FormHitbox {
    fn build(&self, app: &mut App) {
        //Hitbox debug
        app.register_type::<Hitbox>();
        app.register_type::<BaseEntities>();
        app.register_type::<PidInfo>();
        app.register_type::<Offset>();
        // Create hitbox
        app.add_systems(
            OnEnter(StateSpawnScene::Done),
            (spawn_simple_colliders, spawn_hitbox_weapon),
        );
        app.add_systems(
            FixedPostUpdate,
            colliders_look_at
                .run_if(all_chars_created)
                .after(TransformSystem::TransformPropagate),
        );
    }
}
