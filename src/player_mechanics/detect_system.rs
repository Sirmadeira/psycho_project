use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_rapier3d::prelude::*;

use crate::form_player::setup_entities::*;
use crate::player_mechanics::{Grounded, StatusEffectDash, StatusEffectStun,StatusIdle};
use crate::treat_animations::lib::AnimationType;
use crate::form_hitbox::setup_entities::*;
use crate::form_world::setup_entities::*;
use std::borrow::BorrowMut;

use super::StatusEffectAttack;

// These system can add and remove status as long as they are not time based

pub fn detect_hits_body_weapon(
    mut collision_events: EventReader<CollisionEvent>,
    weapon_col: Query<&WeaponCollider>,
    body_col: Query<&Hitbox, Without<WeaponCollider>>,
    base_skeleton: Query<&BaseSkeleton>,
    mut health: Query<&mut Health>,
    parent: Query<&Parent>,
) {
    for mut event in collision_events.read() {
        match event.borrow_mut() {
            CollisionEvent::Started(entity1, entity2, _) => {
                // Check if entity1 is a weapon and entity2 is a body, or vice versa
                let weapon_body_pairs = [(entity1, entity2), (entity2, entity1)];

                for &(weapon_entity, body_entity) in &weapon_body_pairs {
                    if weapon_col.get(*weapon_entity).is_ok() && body_col.get(*body_entity).is_ok()
                    {
                        let skeleton_1 = base_skeleton
                            .get(*weapon_entity)
                            .expect("To be pointing to a skeleton")
                            .0;

                        let skeleton_2 = base_skeleton
                            .get(*body_entity)
                            .expect("To be pointing to a skeleton")
                            .0;

                        // Skip self-collision
                        if skeleton_1 == skeleton_2 {
                            continue;
                        }
                        let player = parent
                            .get(skeleton_1)
                            .expect("Skeleton to have player parented")
                            .get();
                        if let Ok(mut player_hp) = health.get_mut(player) {
                            player_hp.0 -= 1;
                            println!("HITTTOOO");
                        } else {
                            println!("Failed to get player health");
                        }
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}

pub fn detect_hits_wall_weapon(
    wall: Query<&Wall>,
    weapon: Query<&WeaponCollider>,
    base_skeleton: Query<&BaseSkeleton>,
    parent: Query<&Parent>,
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
) {
    for mut event in collision_events.read() {
        match event.borrow_mut() {
            CollisionEvent::Started(entity1, entity2, _) => {
                let pairs = [(entity1, entity2), (entity2, entity1)];
                for &(weapon_entity, wall_entity) in &pairs {
                    if weapon.get(*weapon_entity).is_ok() && wall.get(*wall_entity).is_ok() {
                        let skeleton = base_skeleton
                            .get(*weapon_entity)
                            .expect("To be pointing to a skeleton")
                            .0;

                        let player = parent
                            .get(skeleton)
                            .expect("Skeleton to have player parented")
                            .get();

                        // WHEN WALLBOUNCING YOU RESET JUMP AND DASH
                        commands.entity(player).remove::<StatusEffectDash>();
                        // TODO - RESET JUMP

                        println!("WALLBOUNCE");
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}

// Parry mechanic
pub fn detect_hits_weapon_weapon(
    weapon: Query<&WeaponCollider>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for mut event in collision_events.read() {
        match event.borrow_mut() {
            CollisionEvent::Started(entity1, entity2, _) => {
                let pairs = [(entity1, entity2), (entity2, entity1)];
                for &(first_weapon, second_weapon) in &pairs {
                    if weapon.get(*first_weapon).is_ok() && weapon.get(*second_weapon).is_ok() {
                        println!("I collided with a gun");
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}


pub fn detect_hits_body_ground(
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
                    commands.entity(player).insert(StatusEffectStun {
                        timer: Timer::new(Duration::from_micros(50), TimerMode::Once),
                        played_animation: false,
                    });
                    animation_writer.send(AnimationType::Landing);
                }
            }
        } else {
            commands.entity(player).remove::<Grounded>(); 
        }
    }
}


// Check if player is idle if so it send animation type and adds a component
pub fn detect_if_idle(
    mut q_1: Query<(Entity, &Velocity, &ExternalImpulse, Option<&mut StatusIdle>,Has<StatusEffectAttack>), With<Player>>,
    mut commands: Commands,
    mut animation_writer: EventWriter<AnimationType>,
    time: Res<Time>,
) {
    for (player, vel, imp, opt_status_idle,has_attack) in q_1.iter_mut() {
        if vel.linvel.length() < 0.01 && imp.impulse.length() < 0.01 {
            if let Some(mut status_idle) = opt_status_idle {
                // If a player attacks dont tick after all he is not idle
                if !has_attack{ 
                    status_idle
                    .0
                    .tick(Duration::from_secs_f32(time.delta_seconds()));
                }
                if status_idle.0.just_finished() {
                    animation_writer.send(AnimationType::Idle);
                }
            } else {
                commands.entity(player).insert(StatusIdle(Timer::new(
                    Duration::from_millis(500),
                    TimerMode::Repeating,
                )));
            }
        } else {
            commands.entity(player).remove::<StatusIdle>(); // Remove the idle marker if the player is moving
        }
    }
}

pub fn detect_dead(
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
