use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_rapier3d::prelude::*;

use crate::player_effects::*;
use crate::treat_animations::lib::{AnimationCooldown, AnimationType};
use crate::spawn_game_entities::lib::Ground;
// use crate::treat_animations::lib::AnimationType;

use super::StatusEffectWallBounce;

pub fn check_status_effect(
    time: Res<Time>,
    mut commands: Commands,
    mut q_1: Query<
        (
            Entity,
            Option<&mut StatusEffectDash>,
            Option<&mut AnimationCooldown>,
        ),
        With<Player>,
    >,
) {
    for (ent, status_effect_dash, animation_cd) in q_1.iter_mut() {
        if let Some(mut status) = status_effect_dash {
            status
                .dash_cooldown
                .tick(Duration::from_secs_f32(time.delta_seconds()));
            if status.dash_cooldown.finished() {
                commands.entity(ent).remove::<StatusEffectDash>();
            }
        }

        if let Some(mut cooldown) = animation_cd {
            cooldown
                .0
                .tick(Duration::from_secs_f32(time.delta_seconds()));
            if cooldown.0.finished() {
                commands.entity(ent).remove::<AnimationCooldown>();
            }
        }
    }
}
pub fn check_status_wallbounce(
    time: Res<Time>,
    mut commands: Commands,
    mut q_1: Query<
        (
            Entity,
            Option<&mut StatusEffectWallBounce>,
            Has<StatusEffectDash>,
        ),
        With<Player>,
    >,
    mut q_2: Query<&mut Limit>,
) {
    for (ent, status, has_dashed) in q_1.iter_mut() {
        let mut limit = q_2.get_mut(ent).expect("To have jump limit");
        if let Some(mut status) = status {
            // Resets jump
            *limit = Limit {
                ..Default::default()
            };

            // Reset status effect
            if has_dashed {
                commands.entity(ent).remove::<StatusEffectDash>();
            }

            //Tick cooldown
            status
                .bounce_duration
                .tick(Duration::from_secs_f32(time.delta_seconds()));
            if status.bounce_duration.finished() {
                commands.entity(ent).remove::<StatusEffectWallBounce>();
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
    for entity1 in q_1.iter() {
        let entity2 = q_2.get_single().expect("Ground to exist");
        /* Find the contact pair, if it exists, between two colliders. */
        if let Some(contact_pair) = rapier_context.contact_pair(entity1, entity2) {
            // The contact pair exists meaning that the broad-phase identified a potential contact.
            if contact_pair.has_any_active_contact() {
                // The contact pair has active contacts, meaning that it
                // contains contacts for which contact forces were computed.
                commands.entity(entity1).insert(Grounded);
            }
        } else {
            commands.entity(entity1).remove::<Grounded>();
        }
    }
}

// Check if player is idle if so it send animation type and adds a component
pub fn check_status_idle(
    q_1: Query<(Entity, &Velocity), With<Player>>,
    mut commands: Commands,
    mut animation_writer: EventWriter<AnimationType>,
) {
    const STOPPED_THRESHOLD: f32 = 0.02; // Define a small threshold for stopping

    for (entity, vel) in q_1.iter() {
        if vel.linvel.length_squared() < STOPPED_THRESHOLD * STOPPED_THRESHOLD {
            // If the linear velocity is below the threshold, consider the player stopped
            animation_writer.send(AnimationType::Idle);
            commands.entity(entity).insert(StatusIdle); // Insert a marker component or handle the idle state
        } else {
            commands.entity(entity).remove::<StatusIdle>(); // Remove the idle marker if the player is moving
        }
    }
}

pub fn check_dead(
    hp_entities: Query<(Entity, &Health)>,
    // mut animation_type_writer: EventWriter<AnimationType>,
) {
    for (entity, &Health(hp)) in hp_entities.iter() {
        if hp == 0 {
            println!("THIS DUDE IS DEAD {:?}", entity);
            // animation_type_writer.send(AnimationType::Dead);
        }
    }
}
