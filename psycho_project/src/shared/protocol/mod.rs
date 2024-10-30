use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::prelude::{App, Plugin};
use lightyear::client::components::ComponentSyncMode;
use lightyear::prelude::*;
use lightyear::utils::avian3d::*;

pub mod lobby_structs;
pub mod player_structs;
pub mod world_structs;

use self::{lobby_structs::*, player_structs::*, world_structs::*};

// Protocol
pub(crate) struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        //Resources
        app.register_resource::<Lobbies>(ChannelDirection::ServerToClient);
        app.register_resource::<PlayerBundleMap>(ChannelDirection::ServerToClient);

        // Messages when starting game and just connection
        app.register_message::<StartGame>(ChannelDirection::ServerToClient);
        app.register_message::<SendBundle>(ChannelDirection::ServerToClient);
        // Message start match related
        app.register_message::<EnterLobby>(ChannelDirection::ClientToServer);
        app.register_message::<ExitLobby>(ChannelDirection::ClientToServer);
        // Messages related to visuals
        app.register_message::<SaveVisual>(ChannelDirection::ClientToServer);
        app.register_message::<ChangeChar>(ChannelDirection::Bidirectional);

        // Components
        // Player Components
        app.register_component::<PlayerId>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_component::<PlayerVisuals>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);
        // This specific component has utils based on lightyear TODO - MAYBE MIGRATE
        app.register_component::<Position>(ChannelDirection::Bidirectional)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation_fn(position::lerp)
            .add_correction_fn(position::lerp);

        // app.register_component::<PlayerPosition>(ChannelDirection::ServerToClient)
        //     .add_prediction(ComponentSyncMode::Full)
        //     .add_interpolation(ComponentSyncMode::Full)
        //     .add_linear_interpolation_fn();

        // World components
        app.register_component::<FloorMarker>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);
        // Channels
        app.add_channel::<Channel1>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });
    }
}
