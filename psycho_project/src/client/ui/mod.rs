use bevy::prelude::*;

pub mod inventory_screen;
pub mod lobby_screen;
mod main_screen;

use self::{inventory_screen::*, lobby_screen::LobbyPlugin, main_screen::*};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainMenuPlugin);
        app.add_plugins(LobbyPlugin);
        app.add_plugins(InventoryPlugin);
    }
}
