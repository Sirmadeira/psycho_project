use bevy::prelude::*;
use bevy_atmosphere::prelude::AtmosphereModel;
use bevy_atmosphere::prelude::Nishita;

use crate::MyAppState;

mod lighting_mechanics;
pub mod setup_entities;

use self::{lighting_mechanics::*, setup_entities::*};

pub struct FormWorld;

impl Plugin for FormWorld {
    fn build(&self, app: &mut App) {
        // Creating world
        app.add_systems(OnEnter(MyAppState::InGame), (spawn_floor, spawn_wall));
        // Doing it ciclic lightining
        app.add_systems(Update, daylight_cycle);
        // Atmospheric resources - To config later https://docs.rs/bevy_atmosphere/latest/bevy_atmosphere/collection/nishita/struct.Nishita.html
        app.insert_resource(AtmosphereModel::new(Nishita {
            sun_intensity: 11.0,
            ..default()
        }));
    }
}
