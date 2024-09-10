//! The client plugin.
//! This plugin will act as the entire client meaning every single plugin that relies on the user computer is gonna run here

use bevy::prelude::*;

mod core_systems;
mod create_char;
mod load_assets;

use self::core_systems::CorePlugin;
use self::create_char::CreateCharPlugin;
use self::load_assets::LoadingAssetsPlugin;

// Centralization of plugins
pub struct ExampleClientPlugin;

impl Plugin for ExampleClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CorePlugin);

        // Self made plugins
        app.add_plugins(LoadingAssetsPlugin);
        app.add_plugins(CreateCharPlugin);
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum MyAppState {
    #[default]
    LoadingAssets,
    Loaded,
}
