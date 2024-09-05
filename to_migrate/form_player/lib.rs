use bevy::prelude::*;

use bevy::time::Stopwatch;

//Player
// Marker component - Basically the rigid body that will move the player
#[derive(Component)]
pub struct Player;

// Marker just to easily check other players
#[derive(Component)]
pub struct SidePlayer;

// Amount of jumps you can have
#[derive(Reflect, Component, Debug)]
pub struct Limit {
    pub jump_limit: u8,
}

impl Default for Limit {
    fn default() -> Self {
        Self { jump_limit: 2 }
    }
}

// Kind of a simple pid
#[derive(Reflect, Component, Debug)]
pub struct PlayerVel {
    pub ang_vel: f32,
    pub linvel: f32,
    pub jump_vel: f32,
    pub dash_vel: f32,
}

impl Default for PlayerVel {
    fn default() -> Self {
        Self {
            ang_vel: 600.0,
            linvel: 20.0,
            jump_vel: 20.0,
            dash_vel: 200.0,
        }
    }
}

#[derive(Component, Reflect, Debug)]
pub struct Health(pub i8);

impl Default for Health {
    fn default() -> Self {
        Self(10)
    }
}

// Times the dash for each key
#[derive(Reflect, Component, Debug)]
pub struct DashTimers {
    pub up: Stopwatch,
    pub down: Stopwatch,
    pub left: Stopwatch,
    pub right: Stopwatch,
}

impl Default for DashTimers {
    fn default() -> Self {
        Self {
            up: Stopwatch::new(),
            down: Stopwatch::new(),
            left: Stopwatch::new(),
            right: Stopwatch::new(),
        }
    }
}

// // Gives me the direction of the player is supposed to be attacking from
#[derive(Component, Reflect)]
pub struct StateOfAttack {
    pub attack_states: Vec<&'static str>,
    pub active_attack: &'static str,
    pub index: u8,
}

impl Default for StateOfAttack {
    fn default() -> Self {
        Self {
            attack_states: vec!["LeftAttack", "RightAttack", "FrontAttack", "BackAttack"],
            active_attack: "LeftAttack",
            index: 0,
        }
    }
}

impl StateOfAttack {
    // Function to get attack value based on index
    pub fn get_attack(&self) -> Option<&'static str> {
        self.attack_states.get(self.index as usize).copied()
    }
}

// Marker component - Tells me which is the collider to check for ground
#[derive(Component)]
pub struct PlayerGroundCollider;