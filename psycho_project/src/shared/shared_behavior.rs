//! Here lies every single function that should occur both to server and client. And structs for no
//! It is important to understand when you move something in client you should also try to move it in server, with the same characteristic as in client. Meaning the same input
//! As that will avoid rollbacks and mispredictions, so in summary if client input event -> apply same function -> dont do shit differently
use crate::shared::protocol::player_structs::{Inputs, PlayerPosition};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;





/// Depending on input both server and client need to be moved accordingly
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
    let interpolation_speed = 10.0;
    for (mut transform, player_position) in player.iter_mut() {
        let target_position = player_position.0;
        let current_position = transform.translation;
        transform.translation =
            current_position.lerp(target_position, interpolation_speed * time.delta_seconds());
    }
}
