//! Systems correlated to starting game will occur here
// TODO - DESPAWN EVERYTHING START A NEW

use crate::client::form_player::BodyPartMap;
use crate::client::ui::inventory_screen::ScreenInventory;
use crate::client::ui::lobby_screen::ScreenLobby;
use crate::client::MyAppState;
use crate::shared::protocol::lobby_structs::StartGame;
use bevy::prelude::*;
use lightyear::client::events::MessageEvent;
use lightyear::shared::replication::components::Controlled;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listener_start_game);
        app.add_systems(OnEnter(MyAppState::Game), despawn_previous_screen);
    }
}

// Starts the game this and despawn the current ui screen
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

fn despawn_previous_screen(
    current_ui_screen: Query<Entity, Or<(With<ScreenLobby>, With<ScreenInventory>)>>,
    mut commands: Commands,
) {
    info!("Despawning UI screen");
    let ui_screen = current_ui_screen
        .get_single()
        .expect("You should only have one active screen");

    commands.entity(ui_screen).despawn_recursive();
}

// Main player is already pre created because of char customizer, because
fn create_main_player_entity(
    scenes: Res<BodyPartMap>,
    controlled_player: Query<Entity, Added<Controlled>>,
    mut commands: Commands,
) {
    // A player should be only control him self
    if let Ok(main_player) = controlled_player.get_single() {
        for (scene, _) in scenes.0.values() {
            commands.entity(main_player).set_parent(*scene);
        }
    } else {
        error!("Couldnt find controlled played")
    }
}
