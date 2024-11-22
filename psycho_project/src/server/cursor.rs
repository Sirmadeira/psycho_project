use bevy::prelude::*;
use lightyear::server::events::ConnectEvent;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {}
}

/// When player connects spawn a cursor in server with feature associated to him
fn handle_connection(mut connections: EventReader<ConnectEvent>) {
    
}
