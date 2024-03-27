use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use iyes_perf_ui::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod camera;
mod player;
mod world;

use camera::CameraPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;


// Main running function
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Bevy specific diagnostics for fps counter
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        // Bevy fps counter
        .add_plugins(PerfUiPlugin)
        // Physics plugin
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        // Bevy debugger
        .add_plugins(WorldInspectorPlugin::new())
        // Cameraa Plugin
        .add_plugins(CameraPlugin)
        // Starting the scene and lighting
        .add_plugins(WorldPlugin)
        // Player related confids
        .add_plugins(PlayerPlugin)
        .run();
}
