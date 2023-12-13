use bevy::prelude::*;
mod camera;
mod player;
mod world;

use bevy_third_person_camera::ThirdPersonCameraPlugin;
use camera::CameraPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;
// Base central
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            CameraPlugin,
            WorldPlugin,
            ThirdPersonCameraPlugin,
        ))
        .run();
}
