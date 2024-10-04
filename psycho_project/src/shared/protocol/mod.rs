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
        // Messages
        app.register_message::<StartGame>(ChannelDirection::ServerToClient);
        app.register_message::<SendBundle>(ChannelDirection::ServerToClient);

        app.register_message::<SearchMatch>(ChannelDirection::ClientToServer);
        app.register_message::<StopSearch>(ChannelDirection::ClientToServer);
        app.register_message::<SaveVisual>(ChannelDirection::ClientToServer);
        // Components
        app.register_component::<PlayerId>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);
        app.register_component::<PlayerVisuals>(ChannelDirection::ServerToClient);
        // Channels
        app.add_channel::<Channel1>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });
    }
}
