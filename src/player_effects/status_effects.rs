use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_rapier3d::prelude::*;

use crate::player_effects::*;
use crate::spawn_game_entities::lib::*;
use crate::treat_animations::lib::AnimationType;
// use crate::treat_animations::lib::AnimationType;

use super::StatusEffectWallBounce;

pub fn check_status_ticker(
    time: Res<Time>,
    mut commands: Commands,
    mut q_1: Query<
        (
            Entity,
            Option<&mut StatusEffectDash>,
            Option<&mut StatusEffectStun>,
            Option<&mut StatusEffectAttack>,
        ),
        With<Player>,
    >,
) {
    for (ent, opt_dash, status_effect_stun, opt_attack) in q_1.iter_mut() {
        if let Some(mut status_dash) = opt_dash {
            status_dash
                .dash_cooldown
                .tick(Duration::from_secs_f32(time.delta_seconds()));
            if status_dash.dash_cooldown.finished() {
                commands.entity(ent).remove::<StatusEffectDash>();
            }
        }

        if let Some(mut cooldown) = status_effect_stun {
            cooldown
                .timer
                .tick(Duration::from_secs_f32(time.delta_seconds()));
            if cooldown.timer.finished() {
                println!("NO longer stunned");
                commands.entity(ent).remove::<StatusEffectStun>();
            }
        }

        if let Some(mut status_attack) = opt_attack {
            status_attack
                .0
                .tick(Duration::from_secs_f32(time.delta_seconds()));
            if status_attack.0.finished() {
                println!("No longer attacking");
                commands.entity(ent).remove::<StatusEffectAttack>();
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
    q_3: Query<(Entity, Has<Grounded>), With<Player>>,
    mut animation_writer: EventWriter<AnimationType>,
) {
    // Player
    let (player, is_grounded) = q_3.get_single().expect("Player to exist");
    // Grabs main collider from player and check if it is colliding with ground
    for entity1 in q_1.iter() {
        let entity2 = q_2.get_single().expect("Ground to exist");
        /* Find the contact pair, if it exists, between two colliders. */
        if let Some(contact_pair) = rapier_context.contact_pair(entity1, entity2) {
            // The contact pair exists meaning that the broad-phase identified a potential contact.
            if contact_pair.has_any_active_contact() {
                if is_grounded {
                    return;
                } else {
                    // Tell me player is grounded
                    commands.entity(player).insert(Grounded);
                    // Handle animation and stuns character

                    animation_writer.send(AnimationType::Landing);
                    commands.entity(player).insert(StatusEffectStun {
                        timer: Timer::new(Duration::from_micros(50), TimerMode::Once),
                        played_animation: false,
                    });
                }
            }
        } else {
            commands.entity(player).remove::<Grounded>();
        }
    }
}

// Check if player is idle if so it send animation type and adds a component
pub fn check_status_idle(
    mut q_1: Query<(Entity, &Velocity, &ExternalImpulse, Option<&mut StatusIdle>), With<Player>>,
    mut commands: Commands,
    mut animation_writer: EventWriter<AnimationType>,
    time: Res<Time>,
) {
    for (player, vel, imp, opt_status_idle) in q_1.iter_mut() {
        if vel.linvel.length() < 0.01 && imp.impulse.length() < 0.01 {
            if let Some(mut status_idle) = opt_status_idle {
                status_idle
                    .0
                    .tick(Duration::from_secs_f32(time.delta_seconds()));
                if status_idle.0.finished() {
                    animation_writer.send(AnimationType::Idle);
                }
            } else {
                commands.entity(player).insert(StatusIdle(Timer::new(
                    Duration::from_micros(100),
                    TimerMode::Once,
                )));
            }
        } else {
            commands.entity(player).remove::<StatusIdle>(); // Remove the idle marker if the player is moving
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
