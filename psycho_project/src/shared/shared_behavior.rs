//! Here lies every single function that should occur both to server and client
use crate::shared::protocol::player_structs::{Inputs, PlayerPosition};
use bevy::prelude::*;

pub(crate) fn shared_movement_behaviour(mut position: Mut<PlayerPosition>, input: &Inputs) {
    const MOVE_SPEED: f32 = 0.1;
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

/// Ensures transform of all player are adjusted
pub fn update_transform(
    mut player: Query<(&mut Transform, &PlayerPosition), With<PlayerPosition>>,
    time: Res<Time>,
) {
    for (mut transform, player_position) in player.iter_mut() {
        transform.translation += player_position.0 * time.delta_seconds();
    }
}
