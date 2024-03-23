use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_third_person_camera::ThirdPersonCameraPlugin;
use iyes_perf_ui::prelude::*;

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
        // Bevy debugger
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(CameraPlugin)
        // Starting the scene and lighting
        .add_plugins(WorldPlugin)
        // Player related confids
        .add_plugins(ThirdPersonCameraPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
