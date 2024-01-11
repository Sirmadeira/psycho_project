use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod camera;
mod gun;
mod player;
mod world;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_third_person_camera::ThirdPersonCameraPlugin;

use camera::CameraPlugin;
use gun::GunPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

// Base central
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            PlayerPlugin,
            GunPlugin,
            CameraPlugin,
            WorldPlugin,
            ThirdPersonCameraPlugin,
            WorldInspectorPlugin::new(),
        ))
        .run();
}
