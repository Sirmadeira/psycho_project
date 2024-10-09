use bevy::prelude::*;

pub mod inventory_screen;
pub mod main_screen;
pub mod pause_screen;

use self::{inventory_screen::*, main_screen::*, pause_screen::PausePlugin};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainMenuPlugin);
        app.add_plugins(PausePlugin);
        app.add_plugins(InventoryPlugin);
    }
}
