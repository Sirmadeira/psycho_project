//! RESPONSIBILITIES - HANDLES ALL MODULAR CHARACTERS CREATIONS AND UPDATES LOBBY RTT

use bevy::prelude::*;
use bevy::render::{mesh::skinning::SkinnedMesh, view::NoFrustumCulling};
use lightyear::client::events::ConnectEvent;
use lightyear::connection::id::ClientId;

mod animations;
mod char_customizer;
mod helpers;

use self::{animations::*, char_customizer::*};

pub struct CreateCharPlugin;

impl Plugin for CreateCharPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EasyClient>();
        // Simple system
        app.add_systems(Startup, spawn_light_bundle);
        // Self made plubings
        app.add_plugins(CustomizeChar);
        app.add_plugins(AnimPlayer);
        // Usefull client systems
        app.add_systems(Update, form_client_id);
        // Debugging RTT
        app.add_systems(Update, disable_culling);
    }
}

// Stores you current client id
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct EasyClient(ClientId);

// When we have a connect event grab the client id
fn form_client_id(mut connection_event: EventReader<ConnectEvent>, mut commands: Commands) {
    for event in connection_event.read() {
        let client_id = event.client_id();
        commands.insert_resource(EasyClient(client_id));
    }
}

fn spawn_light_bundle(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::srgb(0.98, 0.95, 0.87),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 1.0, 5.0)),
        ..default()
    });
}

// Debugger function in animations
pub fn disable_culling(mut commands: Commands, skinned: Query<Entity, Added<SkinnedMesh>>) {
    for entity in &skinned {
        commands.entity(entity).insert(NoFrustumCulling);
    }
}
