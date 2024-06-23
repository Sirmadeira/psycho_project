use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_rapier3d::prelude::*;

use crate::form_hitbox_plugin::lib::{BaseSkeleton, Hitbox, WeaponCollider};
use crate::player_effects_plugin::lib::Health;
use crate::player_effects_plugin::StatusEffectWallBounce;
use crate::treat_animations_plugin::AnimationType;
use crate::world_plugin::Wall;
use std::borrow::BorrowMut;

// I only need this because CollidingEntities, is broken. And i dont need the collisions info. For now
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
                            println!("WHY YOU HITTING YOURSELF");
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
                println!("HEY {:?}{:?}",entity1,entity2);
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

                        commands.entity(player).insert(StatusEffectWallBounce {
                            bounce_duration: Timer::new(
                                Duration::from_secs_f32(2.0),
                                TimerMode::Once,
                            ),
                        });

                        println!("The sensor just collided with wall");
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}

// Check if entity is dead
pub fn check_dead(
    hp_entities: Query<(Entity, &Health)>,
    mut animation_type_writer: EventWriter<AnimationType>,
) {
    for (entity, &Health(hp)) in hp_entities.iter() {
        if hp == 0 {
            println!("THIS DUDE IS DEAD {:?}", entity);
            animation_type_writer.send(AnimationType::Dead);
        }
    }
}
