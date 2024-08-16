use bevy::prelude::*;
use bevy_rapier3d::plugin::PhysicsSet;
use std::time::Duration;

use crate:: MyAppState;

pub mod camera_mechanics;
pub mod sync_camera;
pub mod setup_entities;

use self::{setup_entities::*,camera_mechanics::*, sync_camera::*};

use crate::form_player::*;


pub struct IngameCamera;

impl Plugin for IngameCamera {
    fn build(&self, app: &mut App) {
        // Cicle of the sun configuration
        app.insert_resource(CycleTimer(Timer::new(
            Duration::from_secs(3600),
            TimerMode::Repeating,
        )));
        // Debug camera
        app.register_type::<Zoom>();
        app.register_type::<CamInfo>();
        app.add_systems(OnEnter(MyAppState::InGame), spawn_camera_atmosphere);
        app.add_systems(
            Update,
            (
                toggle_cursor,
                orbit_mouse.run_if(orbit_condition),
                zoom_mouse.run_if(zoom_condition),
            )
                .run_if(player_exists)
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
