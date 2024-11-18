use crate::shared::protocol::ChannelDirection;
use crate::shared::protocol::ComponentSyncMode;
use avian3d::prelude::*;
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
        app.register_component::<BulletMarker>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);
        app.register_type::<Weapon>();
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
            bullet_speed: 50.0,
            cooldown: ((FIXED_TIMESTEP_HZ / 5.0) as u16),
        }
    }
}
/// Utilized for spawning bullet
#[derive(Bundle)]
pub struct BulletBundle {
    position: Position,
    velocity: LinearVelocity,
    marker: BulletMarker,
    lifetime: Lifetime,
}

impl BulletBundle {
    pub fn new(owner: ClientId, position: Vec3, velocity: Vec3, current_tick: Tick) -> Self {
        Self {
            position: Position(position),
            velocity: LinearVelocity(velocity),
            lifetime: Lifetime {
                origin_tick: current_tick,
                lifetime: FIXED_TIMESTEP_HZ as i16 * 2,
            },
            marker: BulletMarker::new(owner),
        }
    }
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BulletMarker {
    pub owner: ClientId,
}
impl BulletMarker {
    pub fn new(owner: ClientId) -> Self {
        Self { owner }
    }
}

// despawns `lifetime` ticks after `origin_tick`
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Lifetime {
    pub origin_tick: Tick,
    /// number of ticks to live for
    pub lifetime: i16,
}
