//! RESPONSIBILITIES - HANDLES ALL MODULAR CHARACTERS CREATIONS AND UPDATES LOBBY RTT

use bevy::prelude::*;
use bevy::render::{mesh::skinning::SkinnedMesh, view::NoFrustumCulling};

mod animations;
mod camera;
mod char_customizer;
mod helpers;
mod start_game;

use self::{animations::*, camera::*, char_customizer::*, start_game::*};

pub struct CreateCharPlugin;

impl Plugin for CreateCharPlugin {
    fn build(&self, app: &mut App) {
        // Simple system
        app.add_systems(Startup, spawn_light_bundle);
        // Self made plugins
        app.add_plugins(PlayerCameraPlugin);
        app.add_plugins(CustomizeCharPlugin);
        app.add_plugins(AnimPlayerPlugin);
        app.add_plugins(InGamePlugin);
        // Debugging RTT
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

// Debugger function in animations
pub fn disable_culling(mut commands: Commands, skinned: Query<Entity, Added<SkinnedMesh>>) {
    for entity in &skinned {
        commands.entity(entity).insert(NoFrustumCulling);
    }
}
