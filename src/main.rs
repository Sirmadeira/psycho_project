use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use iyes_perf_ui::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod camera;
mod player;
mod world;
mod resolution;


use camera::CameraPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;
use resolution::ResolutionPlugin;


// Main running function
fn main() {
    App::new()
        .insert_resource(ResolutionSettings {
            r_large: Vec2::new(2490.0,1376.0),
            large: Vec2::new(1920.0, 1080.0),
            medium: Vec2::new(800.0, 600.0),
            small: Vec2::new(640.0, 360.0),
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(ResolutionPlugin)
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

#[derive(Resource)]
struct ResolutionSettings {
    r_large: Vec2,
    large: Vec2,
    medium: Vec2,
    small: Vec2,
}
