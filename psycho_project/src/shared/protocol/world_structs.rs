//! Here are located every single struct that is synced and envolves world
use avian3d::prelude::*;
use bevy::prelude::*;
use crate::shared::protocol::*;
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
    }
}

/// Marker component utilized in sync to know which entity from server I should replicated
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Component)]
pub struct FloorMarker;
