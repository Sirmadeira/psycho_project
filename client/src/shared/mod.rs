//! This module contains the shared code between the client and the server.
//!
//! The rendering code is here because you might want to run the example in host-server mode, where the server also acts as a client.
//! The simulation logic (movement, etc.) should be shared between client and server to guarantee that there won't be
//! mispredictions/rollbacks.
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy_mod_picking::DefaultPickingPlugins;

pub mod protocol;
use self::protocol::player_structs::*;
use self::protocol::ProtocolPlugin;

#[derive(Clone)]
pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        // the protocol needs to be shared between the client and server
        app.add_plugins(ProtocolPlugin);
        if app.is_plugin_added::<RenderPlugin>() {
            app.add_plugins(DefaultPickingPlugins);
            app.add_systems(Startup, init);
        }
    }
}

// Common initialization both  in server and client
fn init(mut commands: Commands) {
    let common_camera = Camera2dBundle {
        transform: Transform::from_xyz(2.3, 1.1, 0.0),
        ..Default::default()
    };
    commands.spawn(common_camera);
}

// This system defines how we update the player's positions when we receive an input
pub(crate) fn shared_movement_behaviour(mut position: Mut<PlayerPosition>, input: &Inputs) {
    const MOVE_SPEED: f32 = 10.0;
    match input {
        Inputs::Direction(direction) => {
            if direction.up {
                position.y += MOVE_SPEED;
            }
            if direction.down {
                position.y -= MOVE_SPEED;
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

// System that draws the boxes of the player positions.
// The components should be replicated from the server to the client
// pub(crate) fn draw_boxes(mut gizmos: Gizmos, players: Query<(&PlayerPosition, &PlayerColor)>) {
//     for (position, color) in &players {
//         gizmos.rect(
//             Vec3::new(position.x, position.y, 0.0),
//             Quat::IDENTITY,
//             Vec2::ONE * 50.0,
//             color.0,
//         );
//     }
// }
