//! Responsible for displaying in little squares the current items available to client
use bevy::prelude::*;

use crate::client::MyAppState;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MyAppState::Inventory), inventory_screen);
    }
}

fn inventory_screen(mut commands: Commands) {
    commands.spawn((NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        background_color: Color::srgb(0.10, 0.10, 0.10).into(),
        ..default()
    },));
}
