use bevy::{pbr::light_consts::lux::OVERCAST_DAY, prelude::*};
use bevy_atmosphere::prelude::*;

use crate::spawn_game_entities::lib::{CycleTimer, Sun};
use crate::MyAppState;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, daylight_cycle.run_if(in_state(MyAppState::InGame)));
    }
}

fn daylight_cycle(
    mut atmosphere: AtmosphereMut<Nishita>,
    mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    mut timer: ResMut<CycleTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        let t = time.elapsed_seconds_wrapped() / 2.0;
        atmosphere.sun_position = Vec3::new(0., t.sin(), t.cos());

        if let Some((mut light_trans, mut directional)) = query.single_mut().into() {
            light_trans.rotation = Quat::from_rotation_x(-t);
            directional.illuminance = t.sin().max(0.0).powf(2.0) * OVERCAST_DAY;
        }
    }
}
