use bevy::{ecs::entity, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::form_hitbox_plugin::lib::{Hitbox, WeaponHitbox};


pub fn detect_hits(
    mut collision_events: EventReader<CollisionEvent>


){  

    for event in collision_events.read() {
        match event.to_owned() {
            CollisionEvent::Started(entity1,entity2 , _ ) =>{
                println!("{:?}{:?}",entity1,entity2)
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {
        }
    }
    }
}





