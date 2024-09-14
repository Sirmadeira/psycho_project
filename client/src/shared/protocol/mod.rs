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
        // messages
        app.register_message::<StartGame>(ChannelDirection::ServerToClient);
        // components
        app.register_component::<PlayerId>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);
        // channels
        app.add_channel::<Channel1>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });
    }
}