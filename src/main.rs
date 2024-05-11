use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use iyes_perf_ui::prelude::*;

mod camera_plugin;
mod player_plugin;
mod resolution_plugin;
mod world_plugin;
mod asset_loader_plugin;
mod mod_char_plugin;

use camera_plugin::CameraPlugin;
use player_plugin::PlayerPlugin;
use resolution_plugin::ResolutionPlugin;
use world_plugin::WorldPlugin;
use asset_loader_plugin::AssetLoaderPlugin;
use mod_char_plugin::ModCharPlugin;

// Main running function
fn main() {
    App::new()
        .insert_resource(ResolutionSettings {
            r_large: Vec2::new(2490.0, 1376.0),
            large: Vec2::new(1920.0, 1080.0),
            medium: Vec2::new(800.0, 600.0),
            small: Vec2::new(640.0, 360.0),
        })
        .insert_resource(Time::<Fixed>::from_hz(64.0))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
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
        // Player related confids
        .add_plugins(PlayerPlugin)
        // Camera Plugin
        .add_plugins(CameraPlugin)
        // Loads our assets with handles
        .add_plugins(AssetLoaderPlugin)
        // Loads our modular character
        .add_plugins(ModCharPlugin)
        .run();
}

#[derive(Resource)]
struct ResolutionSettings {
    r_large: Vec2,
    large: Vec2,
    medium: Vec2,
    small: Vec2,
}
