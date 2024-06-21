use bevy:: prelude::*;
use bevy_rapier3d::prelude::*;

use crate::form_hitbox_plugin::lib::{Hitbox, WeaponCollider};


pub fn detect_hits(
    mut collision_events: EventReader<CollisionEvent>,
    weapon_col: Query<&WeaponCollider>,
    body_col: Query<&Hitbox, Without<WeaponCollider>>,
) {
    for event in collision_events.read() {
        match event.to_owned() {
            CollisionEvent::Started(entity1, entity2, _) => {
                if (weapon_col.contains(entity1) && body_col.contains(entity2))
                    || (weapon_col.contains(entity2) && body_col.contains(entity1))
                {
                    println!("HITOOOOOOOOO");
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}





