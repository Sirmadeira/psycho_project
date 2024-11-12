//! Here are located every single struct that is synced and envolves world
use crate::shared::protocol::*;
use avian3d::prelude::*;
use bevy::utils::Duration;
use serde::{Deserialize, Serialize};

/// Anything that is general and need to be synced is here
pub struct SharedWorldStructsPlugin;

impl Plugin for SharedWorldStructsPlugin {
    fn build(&self, app: &mut App) {
        app.register_component::<LinearVelocity>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<AngularVelocity>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<Name>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        // Position and Rotation have a `correction_fn` set, which is used to smear rollback errors
        // over a few frames, just for the rendering part in postudpate.
        //
        // They also set `interpolation_fn` which is used by the VisualInterpolationPlugin to smooth
        // out rendering between fixedupdate ticks.
        app.register_component::<Position>(ChannelDirection::Bidirectional)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation_fn(position::lerp)
            .add_correction_fn(position::lerp);

        app.register_component::<Rotation>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation_fn(rotation::lerp)
            .add_correction_fn(rotation::lerp);

        // World components
        app.register_component::<FloorMarker>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        app.register_component::<SunMarker>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        app.register_component::<SunPosition>(ChannelDirection::ServerToClient)
            .add_custom_interpolation(ComponentSyncMode::Full);
    }
}

/// Marker component utilized in sync to know which entity from server I should replicated
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Component)]
pub struct FloorMarker;

/// Markes our sun
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Component)]
pub struct SunMarker;

/// TODO
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Component)]
pub struct SunPosition(Quat);

/// Cycle time of the sun, a simple time that is repeating mode everytime he finished we tick a little bit our server sun position
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct CycleTimer(pub Timer);

impl Default for CycleTimer {
    fn default() -> Self {
        CycleTimer(Timer::new(Duration::from_secs(3600), TimerMode::Repeating))
    }
}
