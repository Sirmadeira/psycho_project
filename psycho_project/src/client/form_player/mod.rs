//! RESPONSIBILITIES - HANDLES ALL MODULAR CHARACTERS CREATIONS AND UPDATES LOBBY RTT

use bevy::prelude::*;
use bevy::render::{mesh::skinning::SkinnedMesh, view::NoFrustumCulling};

mod char_customizer;
mod helpers;
pub mod rtt;

use self::{char_customizer::*, rtt::*};

pub struct CreateCharPlugin;

impl Plugin for CreateCharPlugin {
    fn build(&self, app: &mut App) {
        // Rtt system
        app.add_systems(Startup, spawn_rtt_camera);
        // Simple system
        app.add_systems(Startup, spawn_light_bundle);
        // Self made plubings
        app.add_plugins(CustomizeChar);
        // Necesssity
        app.add_systems(Update, disable_culling);
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

// Sets bones in place of original skeleton
// fn finish_player(
//     player_replicated: Query<Entity, (Added<Replicated>, Added<Controlled>)>,
//     visuals: Query<Entity, With<Visual>>,
//     mut commands: Commands,
// ) {
//     if let Ok(player) = player_replicated.get_single() {
//         info!("Aggregating into one single entity for easy of use");
//         for visual in visuals.iter() {
//             commands.entity(visual).set_parent(player);
//         }
//     }
// }

// Debugger function in animations
pub fn disable_culling(mut commands: Commands, skinned: Query<Entity, Added<SkinnedMesh>>) {
    for entity in &skinned {
        commands.entity(entity).insert(NoFrustumCulling);
    }
}
