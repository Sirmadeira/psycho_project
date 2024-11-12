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
        app.add_plugins(SharedWorldStructsPlugin);
        app.add_plugins(LobbyStructsPlugin);
        app.add_plugins(PlayerStructPlugin);

        // Channels
        app.add_channel::<CommonChannel>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });
        app.add_channel::<UnorderedChannel>(ChannelSettings {
            mode: ChannelMode::UnorderedReliable(ReliableSettings::default()),
            ..default()
        });
    }
}

#[derive(Channel)]
pub struct UnorderedChannel;
