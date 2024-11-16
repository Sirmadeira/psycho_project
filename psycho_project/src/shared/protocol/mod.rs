use bevy::prelude::*;
use bevy::prelude::{App, Plugin};
use lightyear::client::components::ComponentSyncMode;
use lightyear::prelude::*;
use lightyear::utils::avian3d::*;

pub mod lobby_structs;
pub mod player_structs;
pub mod weapon_structs;
pub mod world_structs;

use self::{lobby_structs::*, player_structs::*, weapon_structs::*, world_structs::*};

// Protocol
pub(crate) struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldStructsPlugin);
        app.add_plugins(LobbyStructsPlugin);
        app.add_plugins(PlayerStructPlugin);
        app.add_plugins(WeaponStructPlugin);

        // Channels
        app.add_channel::<CommonChannel>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });
        app.add_channel::<ConstantOrderedChannel>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });
    }
}

// Channels
#[derive(Channel)]
pub struct CommonChannel;

/// Channel utilized for constant sending of updates like sun cycle time
#[derive(Channel)]
pub struct ConstantOrderedChannel;