//! Responsible for changin an user resolution
//!  TODO - SETTING UI, THAT SAVES USER KEYBINDS - ONLY TODO WHEN FINISHED UP ALL MECHANICS
use bevy::prelude::*;

pub struct ChangeResPlugin;
// TODO - SAVE SETTINGS IN SERVER

impl Plugin for ChangeResPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ResolutionSettings::default());
        app.add_systems(Startup, def_resolution);
        app.add_systems(Update, toggle_resolution);
    }
}

/// Stores the various window-resolutions we can select between.
#[derive(Resource)]
struct ResolutionSettings {
    full: Vec2,
    large: Vec2,
    medium: Vec2,
    small: Vec2,
}

impl Default for ResolutionSettings {
    fn default() -> Self {
        Self {
            full: Vec2::new(2560.0, 1440.0),
            large: Vec2::new(1920.0, 1080.0),
            medium: Vec2::new(800.0, 600.0),
            small: Vec2::new(640.0, 360.0),
        }
    }
}

// Sets default resolution runs once
fn def_resolution(mut windows: Query<&mut Window>, resolution: Res<ResolutionSettings>) {
    let mut window = windows.single_mut();
    let res = resolution.medium;
    window.resolution.set(res.x, res.y);
}

// Makes it so resolutions are interchangeable
fn toggle_resolution(
    keys: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window>,
    resolution: Res<ResolutionSettings>,
) {
    let mut window = windows.single_mut();

    if keys.just_pressed(KeyCode::Digit1) {
        let res = resolution.small;
        window.resolution.set(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Digit2) {
        let res = resolution.medium;
        window.resolution.set(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Digit3) {
        let res = resolution.large;
        window.resolution.set(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Digit4) {
        let res = resolution.full;
        window.resolution.set(res.x, res.y);
    }
}
