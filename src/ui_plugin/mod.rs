use bevy::prelude::*;

use crate::MyAppState;

use self::{debug_ui::*, lib::*, main_menu::*};

mod debug_ui;
mod lib;
mod main_menu;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_begin_camera);
        app.add_systems(OnEnter(MyAppState::MainMenu), (spawn_menu, spawn_debug));
        app.add_systems(Update, start_button);
    }
}

// This occurs first that is why is separated
fn spawn_begin_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(UiCamera);
}
