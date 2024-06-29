
use bevy::prelude::*;
use bevy_rapier3d::plugin::PhysicsSet;
use crate::player_effects_plugin::player_exists;


pub mod lib;
pub mod spawn_entities;
pub mod sync_camera;
pub mod camera_mechanics;


use self::{lib::*,camera_mechanics::*,spawn_entities::*,sync_camera::*};


pub struct IngameCameraPlugin;

impl Plugin for IngameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CamInfo>();
        app.register_type::<Zoom>();
        app.add_systems(Startup, spawn_camera);
        app.add_systems(
            Update,
            (
                toggle_cursor,
                orbit_mouse.run_if(orbit_condition),
                zoom_mouse.run_if(zoom_condition),
            )
                .chain(),
        );
        app.add_systems(
            PostUpdate,
            sync_player_camera
                .run_if(player_exists)
                .after(PhysicsSet::StepSimulation),
        );
    }
}


// Conditions
// only run the orbit system if the cursor lock is disabled
fn orbit_condition(cam_q: Query<&CamInfo>) -> bool {
    let Ok(cam) = cam_q.get_single() else {
        return true;
    };
    return cam.cursor_lock_active;
}

// only zoom if zoom is enabled & the cursor lock feature is enabled & active
fn zoom_condition(cam_q: Query<&CamInfo, With<CamInfo>>) -> bool {
    let Ok(cam) = cam_q.get_single() else {
        return false;
    };
    return cam.zoom_enabled && cam.cursor_lock_active;
}
