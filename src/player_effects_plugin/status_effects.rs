use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_rapier3d::prelude::*;

use crate::player_effects_plugin::{Grounded, Player, PlayerGroundCollider, StatusEffectDash};
use crate::world_plugin::Ground;

pub fn check_status_effect(
    time: Res<Time>,
    mut commands: Commands,
    mut q_1: Query<(Entity, Option<&mut StatusEffectDash>), With<Player>>,
) {
    for (ent, status) in q_1.iter_mut() {
        if let Some(mut status) = status {
            status
                .dash_duration
                .tick(Duration::from_secs_f32(time.delta_seconds()));
            if status.dash_duration.finished() {
                commands.entity(ent).remove::<StatusEffectDash>();
            }
        } else {
            return;
        }
    }
}

pub fn check_status_grounded(
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
    q_1: Query<Entity, With<PlayerGroundCollider>>,
    q_2: Query<Entity, With<Ground>>,
) {
    // Grabs every hitbox and check if any of them are touching the ground.
    for entity1 in q_1.iter(){
        let entity2 = q_2.get_single().expect("Ground to exist");
        /* Find the contact pair, if it exists, between two colliders. */
        if let Some(contact_pair) = rapier_context.contact_pair(entity1, entity2) {
            // The contact pair exists meaning that the broad-phase identified a potential contact.
            if contact_pair.has_any_active_contacts() {
                // The contact pair has active contacts, meaning that it
                // contains contacts for which contact forces were computed.
                commands.entity(entity1).insert(Grounded);
            }
        } else {
            commands.entity(entity1).remove::<Grounded>();
        }
    }
}
