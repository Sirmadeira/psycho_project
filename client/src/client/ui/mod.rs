use crate::client::MyAppState;
use bevy::prelude::*;

mod lobby_screen;
mod main_screen;

use self::{lobby_screen::*, main_screen::*};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MyAppState::MainMenu), create_character_customizer);
        // app.add_systems(Startup, spawn_begin_camera);
        // app.add_systems(OnEnter(MyAppState::MainMenu), menu_screen);
        // app.add_systems(OnEnter(MyAppState::Lobby), lobby_screen);
        // app.add_systems(Update, start_button.run_if(in_state(MyAppState::MainMenu)));
        // app.add_systems(Update, exit_button.run_if(in_state(MyAppState::MainMenu)));
        // app.add_systems(Update, connect_button.run_if(in_state(MyAppState::Lobby)));
        // app.add_systems(Update, scrolling_list.run_if(in_state(MyAppState::Lobby)));
        // app.add_systems(Update, display_matches.run_if(in_state(MyAppState::Lobby)));
    }
}

#[derive(Component)]
pub struct UiCamera;

fn spawn_begin_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default()).insert(UiCamera);
}
