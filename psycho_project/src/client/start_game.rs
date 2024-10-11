//! Systems correlated to starting game will occur here
use crate::client::ui::inventory_screen::ScreenInventory;
use crate::client::ui::main_screen::ScreenMainMenu;
use crate::client::ui::pause_screen::ScreenPause;
use crate::client::MyAppState;
use crate::shared::protocol::lobby_structs::StartGame;
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use lightyear::client::events::MessageEvent;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MyAppState::Game), despawn_useless_entities);
        app.add_systems(Update, listener_start_game);
        app.add_systems(Update, set_pause_screen);
    }
}

/// Starts the game this and despawn the current ui screen
fn listener_start_game(
    mut events: EventReader<MessageEvent<StartGame>>,
    mut next_state: ResMut<NextState<MyAppState>>,
) {
    for event in events.read() {
        let content = event.message();
        info!("Start game for lobby {}", content.lobby_id);
        next_state.set(MyAppState::Game);
    }
}

/// Despawns ui screens, despawn rtt cameras and if anything else needs despawnign he your boy
fn despawn_useless_entities(
    current_ui_screen: Query<
        Entity,
        Or<(
            With<ScreenPause>,
            With<ScreenInventory>,
            With<ScreenMainMenu>,
        )>,
    >,
    rtt_cameras: Query<Entity, With<PanOrbitCamera>>,
    mut commands: Commands,
) {
    info!("Despawning UI screen");

    if let Ok(ui_screen) = current_ui_screen.get_single() {
        commands.entity(ui_screen).despawn_recursive();
    } else {
        warn!("Didnt find any screen to depspawn")
    }
    if let Ok(rtt_camera) = rtt_cameras.get_single() {
        commands.entity(rtt_camera).despawn();
    }
}

fn set_pause_screen(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<MyAppState>>,
    mut next_state: ResMut<NextState<MyAppState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(match current_state.get() {
            MyAppState::Game => MyAppState::Pause,
            MyAppState::Pause => MyAppState::Game,
            _ => MyAppState::Game,
        });
    }
}
