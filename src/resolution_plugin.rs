use bevy::prelude::*;

#[derive(Resource)]
struct ResolutionSettings {
    r_large: Vec2,
    large: Vec2,
    medium: Vec2,
    small: Vec2,
}

pub struct ResolutionPlugin;

impl Plugin for ResolutionPlugin {
    fn build(&self, app: &mut App) {
        // Resolution related resources
        app.insert_resource(ResolutionSettings {
            r_large: Vec2::new(2490.0, 1376.0),
            large: Vec2::new(1920.0, 1080.0),
            medium: Vec2::new(800.0, 600.0),
            small: Vec2::new(640.0, 360.0),
        });
        app.add_systems(Startup, setup_ui);
        app.add_systems(Update, toggle_resolution);
    }
}

// Spawns the UI
fn setup_ui(mut cmd: Commands) {
    // Node that fills entire background
    cmd.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            ..default()
        },
        ..default()
    });
}

/// This system shows how to request the window to a new resolution
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
        let res = resolution.r_large;
        window.resolution.set(res.x, res.y);
    }
}
