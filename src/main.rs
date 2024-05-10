use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use iyes_perf_ui::prelude::*;

mod camera_plugin;
mod player_plugin;
mod resolution_plugin;
mod world_plugin;
mod asset_loader_plugin;
mod modular_character_plugin;

use camera_plugin::CameraPlugin;
use player_plugin::PlayerPlugin;
use resolution_plugin::ResolutionPlugin;
use world_plugin::WorldPlugin;
use modular_character_plugin::ModularCharacterPlugin;
use asset_loader_plugin::AssetLoaderPlugin;


// Main running function
fn main() {
    App::new()
        .insert_resource(ResolutionSettings {
            r_large: Vec2::new(2490.0, 1376.0),
            large: Vec2::new(1920.0, 1080.0),
            medium: Vec2::new(800.0, 600.0),
            small: Vec2::new(640.0, 360.0),
        })
        // Default Plugins
        .add_plugins(DefaultPlugins)
        // Plugin to adjust resolution size
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
        // Starting the scene and lighting
        .add_plugins(WorldPlugin)
        // Player related configs
        .add_plugins(PlayerPlugin)
        // Camera Plugin
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(ModularCharacterPlugin)
        .run();
}

#[derive(Resource)]
struct ResolutionSettings {
    r_large: Vec2,
    large: Vec2,
    medium: Vec2,
    small: Vec2,
}
