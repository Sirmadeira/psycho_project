use bevy::prelude::*;
use bevy::prelude::{App, Plugin};

use lightyear::client::components::ComponentSyncMode;
use lightyear::prelude::*;

pub mod lobby_structs;
pub mod player_structs;

use self::{lobby_structs::*, player_structs::*};

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
        app.register_component::<PlayerId>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_component::<PlayerVisuals>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_component::<PlayerPosition>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation(ComponentSyncMode::Full)
            .add_linear_interpolation_fn();

        // Channels
        app.add_channel::<Channel1>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });
    }
}
