use bevy::prelude::*;
use bevy::transform::TransformSystem;

pub mod follow_along;

use self::follow_along::*;
use crate::spawn_game_entities::all_chars_created;

pub struct FormHitbox;

impl Plugin for FormHitbox {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedPostUpdate,
            colliders_look_at
                .run_if(all_chars_created)
                .after(TransformSystem::TransformPropagate),
        );
    }
}
