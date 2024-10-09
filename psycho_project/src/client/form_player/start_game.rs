//! Systems correlated to starting game will occur here
// TODO - DESPAWN EVERYTHING START A NEW

use crate::client::essentials::EasyClient;
use crate::client::form_player::BodyPartMap;
use crate::client::ui::main_screen::ScreenMainMenu;
use crate::client::MyAppState;
use crate::shared::protocol::lobby_structs::StartGame;
use crate::shared::protocol::player_structs::PlayerBundleMap;
use bevy::prelude::*;
// use bevy_panorbit_camera::PanOrbitCamera;
use lightyear::client::events::MessageEvent;
use lightyear::prelude::client::Predicted;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listener_start_game);
        app.add_systems(OnEnter(MyAppState::Game), despawn_useless_entities);

        app.add_systems(Update, goto_lobby);

        // Observe when to create player
        app.observe(create_main_player);
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
    current_ui_screen: Query<Entity, With<ScreenMainMenu>>,
    // rtt_cameras: Query<Entity, With<PanOrbitCamera>>,
    mut commands: Commands,
) {
    info!("Despawning UI screen");
    let ui_screen = current_ui_screen
        .get_single()
        .expect("You should only have one active screen");

    // for rtt_camera in rtt_cameras.iter() {
    //     commands.entity(rtt_camera).despawn();
    // }

    commands.entity(ui_screen).despawn_recursive();
}

fn goto_lobby(keys: Res<ButtonInput<KeyCode>>, mut my_app_state: ResMut<NextState<MyAppState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        my_app_state.set(MyAppState::Lobby);
    }
}

/// Main player is already pre created because of char customizer, so we just grab his scenes and boom boom
fn create_main_player(
    trigger: Trigger<OnInsert, Predicted>,
    easy_client: Res<EasyClient>,
    player_bundle_map: Res<PlayerBundleMap>,
    body_part_map: Res<BodyPartMap>,
    mut commands: Commands,
) {
    // A player should be only control him self

    let main_player = trigger.entity();
    let client_id = easy_client.0;

    if let Some(player_bundle) = player_bundle_map.0.get(&client_id) {
        let main_player_scenes = player_bundle.visuals.clone();

        info!("Inserting name into main player");
        commands
            .entity(main_player)
            .insert(SpatialBundle {
                transform: Transform::from_xyz(0.5, 0.0, 0.0),
                ..default()
            })
            .insert(Name::new("MainPlayer"));

        for visual in main_player_scenes.iter_visuals() {
            if let Some(body_part) = body_part_map.0.get(visual) {
                commands.entity(*body_part).set_parent(main_player);
            } else {
                error!(
                    "Couldnt grab the following asset from main player {}",
                    visual
                )
            }
        }
    }
}

//TODO - MAKE REGRESSION MECHANIC PAUSE MENU AND SUCH

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
