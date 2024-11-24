use crate::client::player::CamInfo;
use crate::client::player::ClientPlayerEntityMap;
use crate::shared::protocol::player_structs::ClientInfoBundle;
use crate::shared::protocol::player_structs::PlayerLookAt;
use crate::shared::shared_physics::InputPhysicsSet;
use avian3d::prelude::*;
use bevy::prelude::*;
use lightyear::client::events::ConnectEvent;
use lightyear::client::prediction::Predicted;
use lightyear::shared::replication::components::Replicating;

use super::PlayerId;
pub struct ClientReplicatePlayerPlugin;

impl Plugin for ClientReplicatePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_client_info);

        app.add_systems(FixedPreUpdate, change_look_at);

        app.add_systems(
            FixedUpdate,
            angvel_to_look_at.in_set(InputPhysicsSet::Input),
        );
    }
}

fn spawn_client_info(mut commands: Commands, mut connection_event: EventReader<ConnectEvent>) {
    for event in connection_event.read() {
        let client_id = event.client_id();
        commands.spawn(ClientInfoBundle::new(client_id, Vec3::ZERO));

        info!("Spawning client side replicated info for {}", client_id);
    }
}

/// Gonna make it so player looks at camera
fn change_look_at(
    mut player: Query<&mut PlayerLookAt, With<Replicating>>,
    cam_q: Query<&Transform, With<CamInfo>>,
) {
    if let Ok(mut player_look_at) = player.get_single_mut() {
        if let Ok(cam_transform) = cam_q.get_single() {
            let new_look_at: Vec3 = cam_transform.forward().into();
            if player_look_at.0 != new_look_at {
                player_look_at.0 = new_look_at;
            }
        }
    }
}

fn angvel_to_look_at(
    mut players: Query<(&mut AngularVelocity, &Position, &Rotation), With<Predicted>>,
    player_look_at: Query<(&PlayerLookAt, &PlayerId), With<PlayerLookAt>>,
    client_map: Res<ClientPlayerEntityMap>,
) {
    for (look_at, player_id) in player_look_at.iter() {
        if let Some(player) = client_map.0.get(&player_id.0) {
            // Shared function with server
        }
    }
}
