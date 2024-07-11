use bevy::prelude::*;
use bevy::transform::TransformSystem;

pub mod follow_along;
pub mod helpers;
pub mod lib;
pub mod spawn_hitbox;

use self::{follow_along::*, lib::*, spawn_hitbox::*};
use crate::mod_char::all_chars_created;
use crate::mod_char::lib::StateSpawnScene;
use crate::player_effects::player_exists;
use crate::MyHitboxSet;

pub struct FormHitbox;

impl Plugin for FormHitbox {
    fn build(&self, app: &mut App) {
        app.register_type::<Hitbox>();
        app.register_type::<BaseEntities>();
        app.register_type::<PidInfo>();
        app.register_type::<Offset>();
        app.add_systems(
            OnEnter(StateSpawnScene::Done),
            (spawn_simple_colliders, spawn_hitbox_weapon).in_set(MyHitboxSet::SpawnEntities),
        );
        app.add_systems(
            FixedPostUpdate,
            colliders_look_at
                .in_set(MyHitboxSet::FollowAlongSkeleton)
                .after(TransformSystem::TransformPropagate),
        );
        app.configure_sets(
            OnEnter(StateSpawnScene::Done),
            MyHitboxSet::SpawnEntities.run_if(all_chars_created),
        );
        app.configure_sets(
            FixedPostUpdate,
            MyHitboxSet::FollowAlongSkeleton.run_if(player_exists),
        );
    }
}
