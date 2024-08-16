use bevy::prelude::*;

use crate::MyAppState;

use self::{debug_ui::*, main_menu::*};

mod debug_ui;
mod main_menu;

pub struct FormUi;

impl Plugin for FormUi {
    fn build(&self, app: &mut App) {
        // Create 2d start camera
        app.add_systems(PreStartup, spawn_begin_camera);
        // Spawn the main ui system
        app.add_systems(OnEnter(MyAppState::MainMenu), (spawn_menu, spawn_debug));
        // Tranfer click to MyAppStateIngame
        app.add_systems(Update, start_button.run_if(in_state(MyAppState::MainMenu)));
    }
}

// This occurs first that is why is separated
fn spawn_begin_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(UiCamera);
}
