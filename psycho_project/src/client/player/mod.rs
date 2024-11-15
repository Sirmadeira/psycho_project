//! RESPONSIBILITIES - HANDLES ALL MODULAR CHARACTERS CREATIONS AND UPDATES LOBBY RTT

use bevy::prelude::*;
use bevy::render::{mesh::skinning::SkinnedMesh, view::NoFrustumCulling};
use physics::PlayerPhysicsPlugin;

mod animations;
mod camera;
mod char_customizer;
mod physics;

use self::{animations::*, camera::*, char_customizer::*};

pub struct CreateCharPlugin;

impl Plugin for CreateCharPlugin {
    fn build(&self, app: &mut App) {
        // Self made plugins
        app.add_plugins(PlayerCameraPlugin);
        app.add_plugins(CustomizeCharPlugin);
        app.add_plugins(AnimPlayerPlugin);
        app.add_plugins(PlayerPhysicsPlugin);

        // Debugging RTT
        app.add_systems(Update, disable_culling);
        
    }
}

// Debugger function in animations
pub fn disable_culling(mut commands: Commands, skinned: Query<Entity, Added<SkinnedMesh>>) {
    for entity in &skinned {
        commands.entity(entity).insert(NoFrustumCulling);
    }
}
