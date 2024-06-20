use bevy::prelude::*;
use bevy::transform::TransformSystem;

pub mod follow_along;
pub mod helpers;
pub mod lib;
pub mod spawn_hitbox;

use crate::form_hitbox_plugin::follow_along::colliders_look_at;
use crate::form_hitbox_plugin::lib::*;
use crate::form_hitbox_plugin::spawn_hitbox::*;
use crate::player_effects_plugin::lib::StatePlayerCreation;

pub struct FormHitboxPlugin;

impl Plugin for FormHitboxPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Hitbox>();
        app.register_type::<BaseEntities>();
        app.register_type::<PidInfo>();
        app.register_type::<Offset>();
        app.add_systems(OnEnter(StatePlayerCreation::Done), spawn_simple_colliders);
        app.add_systems(
            FixedPostUpdate,
            colliders_look_at
                .run_if(in_state(StatePlayerCreation::Done))
                .after(TransformSystem::TransformPropagate),
        );
    }
}
