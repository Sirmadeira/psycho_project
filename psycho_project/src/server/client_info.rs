use bevy::prelude::*;
use lightyear::prelude::{server::*, NetworkTarget};

use crate::shared::shared_physics::REPLICATION_GROUP;
use lightyear::prelude::server::Replicate;
use lightyear::shared::replication::components::Replicated;

pub struct ClientInfoPlugin;

impl Plugin for ClientInfoPlugin {
    fn build(&self, app: &mut App) {
        // Re-adding Replicate components to client-replicated entities must be done in this set for proper handling.
        app.add_systems(
            PreUpdate,
            replicate_client_info.in_set(ServerReplicationSet::ClientReplication),
        );
    }
}

/// Replicates player infos that are calculated by client to other
fn replicate_client_info(
    client_infos: Query<(Entity, &Replicated), Added<Replicated>>,
    mut commands: Commands,
) {
    for (entity, client_info) in client_infos.iter() {
        if let Some(mut e) = commands.get_entity(entity) {
            let client_id = client_info.client_id();
            info!(
                "Changing replication type for client sent info from client {:?}",
                client_id
            );
            let server_replicate = Replicate {
                authority: AuthorityPeer::Client(client_id),
                sync: SyncTarget {
                    prediction: NetworkTarget::AllExceptSingle(client_id),
                    ..default()
                },
                controlled_by: ControlledBy {
                    target: NetworkTarget::Single(client_id),
                    ..default()
                },
                group: REPLICATION_GROUP,
                ..default()
            };
            e.insert(server_replicate);
        }
    }
}
