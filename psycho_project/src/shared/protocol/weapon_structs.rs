use bevy::prelude::*;
use lightyear::shared::tick_manager::Tick;
use serde::{Deserialize, Serialize};
pub struct WeaponStructPlugin;

impl Plugin for WeaponStructPlugin {
    fn build(&self, app: &mut App) {}
}

/// Struct responsible for defining 
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub(crate) struct Weapon {
    pub(crate) last_fire_tick: Tick,
    pub(crate) cooldown: u16,
    pub(crate) bullet_speed: f32,
}

impl Weapon {
    pub(crate) fn new(cooldown: u16) -> Self {
        Self {
            last_fire_tick: Tick(0),
            bullet_speed: 500.0,
            cooldown,
        }
    }
}

//
