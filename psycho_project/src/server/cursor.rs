use bevy::prelude::*;
use lightyear::{
    prelude::{
        server::{AuthorityPeer, SyncTarget},
        NetworkTarget,
    },
    server::events::ConnectEvent,
};

use crate::shared::protocol::player_structs::PlayerLookAt;
use lightyear::prelude::server::Replicate;
pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {}
}

/// When player connects spawn a cursor in server with feature associated to him
fn handle_connection(mut connections: EventReader<ConnectEvent>, mut commands: Commands) {
    for connection in connections.read() {
        let client_id = connection.client_id;

        commands.spawn(PlayerLookAt::default()).insert(Replicate {
            authority: AuthorityPeer::Client(client_id),
            sync: SyncTarget {
                prediction: NetworkTarget::All,
                ..default()
            },
            ..default()
        });
    }
}
