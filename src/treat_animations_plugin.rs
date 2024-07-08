use bevy::prelude::*;
use crate::player_effects_plugin::player_exists;
use crate::MyAppState;

pub struct TreatAnimationsPlugin;

impl Plugin for TreatAnimationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationType>();
        app.add_systems(
            Update,
            state_machine
                .run_if(player_exists)
                .run_if(in_state(MyAppState::InGame)),
        );
    }
}

// Tells me which type of movement i should pass, to avoid multiple arguments or enums
#[derive(Event, Debug)]
pub enum AnimationType {
    // If it is forward backwards and so on
    None,
    WalkForward,
    WalkBackward,
    WalkLeft,
    WalkRight,
    LeftAttack,
    RightAttack,
    BackwardAttack,
    ForwardAttack,
    Defend,
    Jump,
    DashForward,
    DashBackward,
    DashLeft,
    DashRight,
    Dead,
}

fn state_machine(){}