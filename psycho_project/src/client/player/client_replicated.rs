use crate::shared::protocol::player_structs::ClientInfoBundle;
use bevy::prelude::*;
use lightyear::client::events::ConnectEvent;

/// Plugin made to do server culling, could be usefull when making interactions
pub struct ClientReplicatePlayerPlugin;

impl Plugin for ClientReplicatePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_client_info);
    }
}

fn spawn_client_info(mut commands: Commands, mut connection_event: EventReader<ConnectEvent>) {
    for event in connection_event.read() {
        let client_id = event.client_id();
        commands.spawn(ClientInfoBundle::new(client_id, Vec3::ZERO));

        info!("Spawning client side replicated info for {}", client_id);
    }
}
