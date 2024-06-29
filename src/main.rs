use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use form_hitbox_plugin::FormHitboxPlugin;
use iyes_perf_ui::prelude::*;

mod form_hitbox_plugin;
mod ingame_camera_plugin;
mod load_assets_plugin;
mod mod_char_plugin;
mod player_effects_plugin;
mod resolution_plugin;
mod treat_animations_plugin;
mod ui_plugin;
mod world_plugin;

use ingame_camera_plugin::IngameCameraPlugin;
use load_assets_plugin::LoadingAssetsPlugin;
use mod_char_plugin::ModCharPlugin;
use player_effects_plugin::PlayerEffectsPlugin;
use resolution_plugin::ResolutionPlugin;
use treat_animations_plugin::TreatAnimationsPlugin;
use ui_plugin::UiPlugin;
use world_plugin::WorldPlugin;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash,Default)]
pub enum MyAppState {
    #[default]
    Loading,
    MainMenu,
    InGame,
}

// Set responsbile to handle player related configs
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum MyPlayerSet {
    SpawnEntities,
    HandleStatusEffects,
    HandleInputs,
    DetectCollisions,
    SidePhysics,
}
// Set responsible for handling modular characters
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum MyModCharSet {
    SpawnEntities,
    AttachToSkeleton,
}

// Set responsible for handling hitboxes and their existences
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum MyHitboxSet {
    SpawnEntities,
    FollowAlongSkeleton,
}

// Main running function
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Run at the smae timestep as rapier
        .insert_resource(Time::<Fixed>::from_hz(60.0))
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
        // Main menu and debugger
        .add_plugins(UiPlugin)
        // Bevy egui debugger
        .add_plugins(WorldInspectorPlugin::new())
        // Loads our assets with handles
        .add_plugins(LoadingAssetsPlugin)
        // Starting the scene and lighting
        .add_plugins(WorldPlugin)
        // Loads our modular character
        .add_plugins(ModCharPlugin)
        // Forms physical dynamic colliders that will folllow along the transform of the player
        .add_plugins(FormHitboxPlugin)
        // Player movement plugin
        .add_plugins(PlayerEffectsPlugin)
        // Reads animations according to events and make they smooth
        .add_plugins(TreatAnimationsPlugin)
        // Camera Plugin
        .add_plugins(IngameCameraPlugin)
        .run();
}
