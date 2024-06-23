use std::borrow::BorrowMut;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::world_plugin::Wall;
use crate::player_effects_plugin::lib::Health;
use crate::treat_animations_plugin::AnimationType;
use crate::form_hitbox_plugin::lib::{BaseSkeleton, Hitbox, WeaponCollider};


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
                // Check if both entities have a BaseSkeleton component
                let skeleton_1 = match base_skeleton.get(*entity1) {
                    Ok(skeleton) => skeleton.0,
                    Err(_) => {
                        println!("Entity1 does not have a BaseSkeleton");
                        continue;
                    }
                };
                let skeleton_2 = match base_skeleton.get(*entity2) {
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

                // Check if entity1 is a weapon and entity2 is a body, or vice versa
                let weapon_body_pairs = [
                    (entity1, entity2),
                    (entity2, entity1),
                ];

                for &(weapon_entity, body_entity) in &weapon_body_pairs {
                    if weapon_col.get(*weapon_entity).is_ok() && body_col.get(*body_entity).is_ok() {
                        let skeleton = base_skeleton
                            .get(*weapon_entity)
                            .expect("To be pointing to a skeleton")
                            .0;
                        let player = parent.get(skeleton).expect("Skeleton to have player parented").get();
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

pub fn detect_hits_wall_weapon (wall: Query<&Wall>,
                    weapon: Query<&WeaponCollider>,
                    mut collision_events: EventReader<CollisionEvent>,){
            
        for mut event in collision_events.read(){
            match event.borrow_mut() {
                CollisionEvent::Started(entity1, entity2, _) => {println!("This checks if i am consuming the vent")}
                CollisionEvent::Stopped(_, _, _) => {}

            }
        }

        
}




// Check if entity is dead
pub fn check_dead(hp_entities: Query<(Entity,&Health)>,
                mut animation_type_writer: EventWriter<AnimationType>,
                 ){

    for (entity,&Health(hp)) in hp_entities.iter(){
        if hp == 0{
            println!("THIS DUDE IS DEAD {:?}", entity);
            animation_type_writer.send(AnimationType::Dead);
        }
    }
}