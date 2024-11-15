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
        app.register_component::<Weapon>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);
    }
}

/// Struct responsible for defining
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Weapon {
    pub last_fire_tick: Tick,
    pub cooldown: u16,
    pub bullet_speed: f32,
}

impl Weapon {
    pub fn new(cooldown: u16) -> Self {
        Self {
            last_fire_tick: Tick(0),
            bullet_speed: 500.0,
            cooldown,
        }
    }
    pub fn default() -> Self {
        Self {
            last_fire_tick: Tick(0),
            bullet_speed: 500.0,
            cooldown: ((FIXED_TIMESTEP_HZ / 5.0) as u16),
        }
    }
}

//
