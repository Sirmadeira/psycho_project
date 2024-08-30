use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use iyes_perf_ui::prelude::*;


mod form_hitbox;
mod form_ingame_camera;
mod form_modular_char;
mod form_player;
mod form_ui;
mod form_world;
mod load_assets_plugin;

mod player_mechanics;
mod resolution_plugin;
mod treat_animations;

use form_hitbox::FormHitbox;
use form_ingame_camera::FormIngameCamera;
use form_modular_char::FormModularChar;
use form_player::FormPlayer;
use form_ui::FormUi;
use form_world::FormWorld;
use load_assets_plugin::LoadingAssetsPlugin;
use player_mechanics::PlayerMechanics;
use resolution_plugin::ResolutionPlugin;
use treat_animations::TreatAnimations;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum MyAppState {
    #[default]
    Loading,
    CreatingCharacter,
    TranferingAnimations,
    CharacterCreated,
    PlayerCreated,
    MainMenu,
    InGame,
}





mod client;
mod server;
mod shared;

use crate::client::ExampleClientPlugin;
use crate::server::ExampleServerPlugin;
use clap::Parser;

#[derive(Parser, PartialEq, Debug)]
pub enum Cli {
    /// The program will act as server
    Server,
    /// The program will act as a client
    Client,
}


// Main running function - I am gonna have to destroy u
fn main() {
    let cli = Cli::parse();

    let mut app = App::new();

    match cli {
        Cli::Server => app.add_plugins(ExampleServerPlugin),
        Cli::Client => app.add_plugins(ExampleClientPlugin),
    };



    app
        // Really important configs
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        // A simple plugin to adjust screen size
        .add_plugins(ResolutionPlugin)
        // Bevy specific diagnostics for debug
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
        // Loads our assets with handles
        .add_plugins(LoadingAssetsPlugin)
        // Plugin to form a cube that render cool atmosphere shaders
        .add_plugins(AtmospherePlugin)
        // Here endss external plugins

        // Main menu and debugger menu
        .add_plugins(FormUi)
        // Contruct the world
        .add_plugins(FormWorld)
        // Formulates all the game entities to be used
        .add_plugins(FormModularChar)
        // Forms physical dynamic colliders that will folllow along the transform of the player
        .add_plugins(FormHitbox)
        // Create main rigidbody that moves around and such
        .add_plugins(FormPlayer)
        // Player movement and status effects
        .add_plugins(PlayerMechanics)
        // Camera Plugin
        .add_plugins(FormIngameCamera)
        // Reads animations according to events and make them smooth
        .add_plugins(TreatAnimations)
        .run();
}
