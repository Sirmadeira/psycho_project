use bevy::prelude::*;

mod spawn_scenes;

use crate::asset_loader_plugin::AssetLoaderState;
use self::spawn_scenes::spawn_scenes;

pub struct ModCharPlugin;

impl Plugin for ModCharPlugin{
    fn build (&self, app: &mut App) {
        // Will only load after we finished loading the assets
        app.add_systems(OnEnter(AssetLoaderState::Done),spawn_scenes);
    }
}