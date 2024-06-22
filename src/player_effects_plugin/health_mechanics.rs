use bevy:: prelude::*;
use bevy_rapier3d::prelude::*;

use crate::player_effects_plugin::lib::Health;
use crate::form_hitbox_plugin::lib::{Hitbox, WeaponCollider,BaseSkeleton};


fn handle_collision(
    weapon_entity: Entity,
    body_entity: Entity,
    weapon_col: &Query<&WeaponCollider>,
    body_col: &Query<&Hitbox, Without<WeaponCollider>>,
    base_skeleton: &Query<&BaseSkeleton>,
    health: &mut Query<&mut Health>,
    parent: &Query<&Parent>,
) {
    if weapon_col.get(weapon_entity).is_ok() && body_col.get(body_entity).is_ok() {
        let skeleton = base_skeleton
            .get(weapon_entity)
            .expect("To be pointing to a skeleton")
            .0;
        let player = parent
            .get(skeleton)
            .expect("Skeleton to have player")
            .get();
        if let Ok(mut player_hp) = health.get_mut(player) {
            player_hp.0 -= 1;
            println!("HITTTOOO");
        } else {
            println!("Failed to get player health");
        }
    }
}

pub fn detect_hits(
    mut collision_events: EventReader<CollisionEvent>,
    weapon_col: Query<&WeaponCollider>,
    body_col: Query<&Hitbox, Without<WeaponCollider>>,
    base_skeleton: Query<&BaseSkeleton>,
    mut health: Query<&mut Health>,
    parent: Query<&Parent>,
) {
    for event in collision_events.read() {
        match event.to_owned() {
            CollisionEvent::Started(entity1, entity2, _) => {
                // Check if both entities have a BaseSkeleton component
                let skeleton_1 = match base_skeleton.get(entity1) {
                    Ok(skeleton) => skeleton.0,
                    Err(_) => {
                        println!("Entity1 does not have a BaseSkeleton");
                        continue;
                    }
                };
                let skeleton_2 = match base_skeleton.get(entity2) {
                    Ok(skeleton) => skeleton.0,
                    Err(_) => {
                        println!("Entity2 does not have a BaseSkeleton");
                        continue;
                    }
                };

                // Skip self-collision
                if skeleton_1 == skeleton_2 {
                    println!("WHY YOU HITTING YOURSELF");
                    continue;
                }

                // Handle collisions
                handle_collision(entity1, entity2, &weapon_col, &body_col, &base_skeleton, &mut health, &parent);
                handle_collision(entity2, entity1, &weapon_col, &body_col, &base_skeleton, &mut health, &parent);
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}




