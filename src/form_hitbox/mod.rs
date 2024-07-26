use bevy::prelude::*;
use bevy::transform::TransformSystem;

pub mod follow_along;

use self::follow_along::*;
use crate::spawn_game_entities::{all_chars_created,player_exists};
use crate::spawn_game_entities::lib::StateSpawnScene;
use crate::MyHitboxSet;

pub struct FormHitbox;

impl Plugin for FormHitbox {
    fn build(&self, app: &mut App) {
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
