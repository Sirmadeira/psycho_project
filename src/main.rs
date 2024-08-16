use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use form_hitbox::FormHitbox;
use iyes_perf_ui::prelude::*;

mod form_hitbox;
mod form_world;
mod form_camera;
mod load_assets_plugin;
mod player_effects;
mod resolution_plugin;
mod spawn_game_entities;
mod treat_animations;
mod ui_plugin;

use form_world::WorldPlugin;
use form_camera::IngameCamera;
use load_assets_plugin::LoadingAssetsPlugin;
use player_effects::PlayerEffects;
use resolution_plugin::ResolutionPlugin;
use spawn_game_entities::SpawnGameEntities;
use treat_animations::TreatAnimations;
use ui_plugin::UiPlugin;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum MyAppState {
    #[default]
    Loading,
    MainMenu,
    InGame,
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
        // Bevy egui debugger
        .add_plugins(WorldInspectorPlugin::new())
        // Physics plugin
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        // Main menu and debugger
        .add_plugins(UiPlugin)
        // Loads our assets with handles
        .add_plugins(LoadingAssetsPlugin)
        // Plugin to form a cube that render cool atmosphere shaders
        .add_plugins(AtmospherePlugin)
        // Contruct the world
        .add_plugins(WorldPlugin)
        // Formulates all the game entities to be used
        .add_plugins(SpawnGameEntities)
        // Forms physical dynamic colliders that will folllow along the transform of the player
        .add_plugins(FormHitbox)
        // Player movement plugin
        .add_plugins(PlayerEffects)
        // Reads animations according to events and make they smooth
        .add_plugins(TreatAnimations)
        // Camera Plugin
        .add_plugins(IngameCamera)
        .run();
}
