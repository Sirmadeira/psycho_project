use bevy::prelude::*;

mod inventory_screen;
mod lobby_screen;
mod main_screen;

use self::{inventory_screen::*, lobby_screen::LobbyPlugin, main_screen::*};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_begin_camera);
        app.add_plugins(MainMenuPlugin);
        app.add_plugins(LobbyPlugin);
        app.add_plugins(InventoryPlugin);
    }
}

// Marker component tells me who is my main camera - A lot of mechanic in the future gonna be based on it
#[derive(Component)]
pub struct MainCamera;

fn spawn_begin_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default()).insert(MainCamera);
}
