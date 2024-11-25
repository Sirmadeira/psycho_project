//! RESPONSIBILITIES - HANDLES ALL MODULAR CHARACTERS CREATIONS AND UPDATES LOBBY RTT

use bevy::prelude::*;
use bevy::render::{mesh::skinning::SkinnedMesh, view::NoFrustumCulling};
use bevy::utils::HashMap;
use gun::PlayerGunPlugin;
use lightyear::prelude::client::Predicted;
use physics::PlayerPhysicsPlugin;

mod animations;
mod camera;
mod char_customizer;
mod client_replicated;
mod gun;
mod physics;

use lightyear::connection::id::ClientId;

use self::{animations::*, camera::*, char_customizer::*, client_replicated::*};
use crate::shared::protocol::player_structs::*;

pub struct CreateCharPlugin;

impl Plugin for CreateCharPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClientPlayerEntityMap>();

        // Self made plugins
        app.add_plugins(PlayerCameraPlugin);
        // app.add_plugins(ClientReplicatePlayerPlugin);
        app.add_plugins(CustomizeCharPlugin);
        app.add_plugins(AnimPlayerPlugin);
        app.add_plugins(PlayerPhysicsPlugin);
        app.add_plugins(PlayerGunPlugin);

        app.add_systems(Update, fill_player_map);
        // Debugging RTT
        app.add_systems(Update, disable_culling);
    }
}

#[derive(Resource, Clone, Default, Reflect)]
#[reflect(Resource, Default)]
pub struct ClientPlayerEntityMap(pub HashMap<ClientId, Entity>);

fn fill_player_map(
    player_entities: Query<(Entity, &PlayerId), (With<MarkerPlayer>, Added<Predicted>)>,
    mut player_map: ResMut<ClientPlayerEntityMap>,
) {
    for (entity, player_id) in player_entities.iter() {
        player_map.0.insert(player_id.0, entity);
    }
}

// Debugger function in animations
fn disable_culling(mut commands: Commands, skinned: Query<Entity, Added<SkinnedMesh>>) {
    for entity in &skinned {
        commands.entity(entity).insert(NoFrustumCulling);
    }
}
