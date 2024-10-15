//! Here lies every single function that should occur both to server and client
use crate::shared::protocol::player_structs::{Inputs, PlayerPosition};
use bevy::prelude::*;

pub(crate) fn shared_movement_behaviour(mut position: Mut<PlayerPosition>, input: &Inputs) {
    const MOVE_SPEED: f32 = 10.0;
    match input {
        Inputs::Direction(direction) => {
            if direction.forward {
                position.z += MOVE_SPEED;
            }
            if direction.down {
                position.z -= MOVE_SPEED;
            }
            if direction.left {
                position.x -= MOVE_SPEED;
            }
            if direction.right {
                position.x += MOVE_SPEED;
            }
        }
        _ => {}
    }
}
