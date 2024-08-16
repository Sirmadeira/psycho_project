use bevy::prelude::*;
use bevy::transform::TransformSystem;

mod follow_along;
mod helpers;
pub mod setup_entities;

use crate::MyAppState;

use self::{follow_along::*, setup_entities::*};

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
            OnEnter(MyAppState::CharacterCreated),
            (spawn_simple_colliders, spawn_hitbox_weapon),
        );
        app.add_systems(
            FixedPostUpdate,
            colliders_look_at.after(TransformSystem::TransformPropagate),
        );
    }
}
