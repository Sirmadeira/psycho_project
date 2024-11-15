use crate::shared::protocol::ChannelDirection;
use crate::shared::protocol::ComponentSyncMode;
use bevy::prelude::*;
use common::shared::FIXED_TIMESTEP_HZ;
use lightyear::prelude::*;
use lightyear::shared::tick_manager::Tick;
use serde::{Deserialize, Serialize};
pub struct WeaponStructPlugin;

impl Plugin for WeaponStructPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Weapon>();
        app.register_component::<Weapon>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);
    }
}

/// Struct responsible for defining
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
pub struct Weapon {
    /// Tick since we lasted fired weapong
    pub last_fire_tick: Tick,
    // Our cooldown is based around timestep in this case 64 hz divide by 5 wanna be able to shoot continously just increase the
    // divisor
    pub cooldown: u16,
    // Speed of bullet from this weapon
    pub bullet_speed: f32,
}

impl Weapon {
    pub fn default() -> Self {
        Self {
            last_fire_tick: Tick(0),
            bullet_speed: 500.0,
            cooldown: ((FIXED_TIMESTEP_HZ / 5.0) as u16),
        }
    }
}

//
