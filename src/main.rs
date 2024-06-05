use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use form_hitbox_plugin::FormHitboxPlugin;
use iyes_perf_ui::prelude::*;

mod asset_loader_plugin;
mod camera_plugin;
mod form_hitbox_plugin;
mod mod_char_plugin;
mod player_movement_plugin;
mod resolution_plugin;
mod world_plugin;

use asset_loader_plugin::AssetLoaderPlugin;
use camera_plugin::CameraPlugin;
use mod_char_plugin::ModCharPlugin;
use player_movement_plugin::PlayerMovementPlugin;
use resolution_plugin::ResolutionPlugin;
use world_plugin::WorldPlugin;

// Main running function
fn main() {
    App::new()
        // Resolution related resources
        .insert_resource(ResolutionSettings {
            r_large: Vec2::new(2490.0, 1376.0),
            large: Vec2::new(1920.0, 1080.0),
            medium: Vec2::new(800.0, 600.0),
            small: Vec2::new(640.0, 360.0),
        })
        // Thing I may want  to change later
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        // A simple plugin to adjust screen size
        .add_plugins(ResolutionPlugin)
        // Bevy specific diagnostics for fps counter
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        // Bevy perf UI
        .add_plugins(PerfUiPlugin)
        // Physics plugin
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        // Bevy debugger
        .add_plugins(WorldInspectorPlugin::new())
        // Starting the scene and lighting
        .add_plugins(WorldPlugin)
        // Loads our assets with handles
        .add_plugins(AssetLoaderPlugin)
        // Loads our modular character
        .add_plugins(ModCharPlugin)
        // Player related confids
        .add_plugins(PlayerMovementPlugin)
        // Forms physical dynamic colliders that will folllow along the transform of the player
        .add_plugins(FormHitboxPlugin)
        // Camera Plugin
        .add_plugins(CameraPlugin)
        .run();
}

#[derive(Resource)]
struct ResolutionSettings {
    r_large: Vec2,
    large: Vec2,
    medium: Vec2,
    small: Vec2,
}
