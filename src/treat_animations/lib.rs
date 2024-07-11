

use bevy::prelude::*;

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

