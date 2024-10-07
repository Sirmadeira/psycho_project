//! Systems correlated to starting game will occur here
// TODO - DESPAWN EVERYTHING START A NEW

use crate::client::form_player::BodyPartMap;
use crate::client::ui::inventory_screen::ScreenInventory;
use crate::client::ui::lobby_screen::ScreenLobby;
use crate::client::MyAppState;
use crate::shared::protocol::lobby_structs::StartGame;
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use lightyear::client::events::MessageEvent;
use lightyear::client::prediction::Predicted;
use lightyear::shared::replication::components::Controlled;
pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listener_start_game);
        app.add_systems(OnEnter(MyAppState::Game), despawn_useless_entities);
        // app.add_systems(
        //     Update,
        //     create_main_player.run_if(in_state(MyAppState::Game)),
        // );
        // app.add_systems(Update, set_camera_focus.run_if(in_state(MyAppState::Game)));
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

// Despawns ui screens, despawn rtt cameras and if anything else needs despawnign he your boy
fn despawn_useless_entities(
    current_ui_screen: Query<Entity, Or<(With<ScreenLobby>, With<ScreenInventory>)>>,
    rtt_cameras: Query<Entity, With<PanOrbitCamera>>,
    mut commands: Commands,
) {
    info!("Despawning UI screen");
    let ui_screen = current_ui_screen
        .get_single()
        .expect("You should only have one active screen");

    for rtt_camera in rtt_cameras.iter() {
        commands.entity(rtt_camera).despawn();
    }

    commands.entity(ui_screen).despawn_recursive();
}

// Main player is already pre created because of char customizer, because
// fn create_main_player(
//     scenes: Res<BodyPartMap>,
//     controlled_player: Query<Entity, (Added<Predicted>, Added<Controlled>)>,
//     mut commands: Commands,
// ) {
//     // A player should be only control him self

//     if let Ok(main_player) = controlled_player.get_single() {
//         info!("Found player");
//         for (scene, _) in scenes.0.values() {
//             commands
//                 .entity(main_player)
//                 .insert(SpatialBundle::default());
//             commands.entity(*scene).set_parent(main_player);
//         }
//     }
// }

// // When player enter in game, the camera should orbit around him later we make it so it follows him
// fn set_camera_focus(
//     controlled_player: Query<&Transform, (Added<Controlled>, Added<Replicated>)>,
//     camera: Query<Entity, With<MainCamera>>,
//     mut commands: Commands,
// ) {
//     info_once!("Getting player");
//     // let transform = controlled_player.get_single().expect("One sole player");
//     let cam = camera.get_single().expect("For one lonely camera");
// }
