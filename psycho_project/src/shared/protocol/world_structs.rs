//! Here are located every single struct that is synced and envolves world
use crate::shared::protocol::*;
use avian3d::prelude::*;
use bevy::utils::Duration;
use serde::{Deserialize, Serialize};

/// Anything that is general and need to be synced is here
pub struct WorldStructsPlugin;

impl Plugin for WorldStructsPlugin {
    fn build(&self, app: &mut App) {
        // Shared debuggin
        app.register_type::<CycleTimer>();

        // Resources
        app.register_resource::<CycleTimer>(ChannelDirection::ServerToClient);

        // Physics
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
    }
}

/// Marker component utilized in sync to know which entity from server I should replicated
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Component)]
pub struct FloorMarker;

/// Cycle time of the sun, a simple time that is repeating mode everytime he finished we tick a little bit our server sun position
#[derive(Resource, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
#[reflect(Resource, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct CycleTimer(pub Timer);

impl Default for CycleTimer {
    fn default() -> Self {
        CycleTimer(Timer::new(
            // Default cycle duration is 24 hours (in seconds), but this can be changed
            Duration::from_secs(24),
            TimerMode::Repeating,
        ))
    }
}
