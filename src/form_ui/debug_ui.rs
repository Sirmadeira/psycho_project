use bevy::prelude::*;
use iyes_perf_ui::entries::PerfUiCompleteBundle;

pub fn spawn_debug(mut commands: Commands) {
    commands.spawn(PerfUiCompleteBundle::default());
}
