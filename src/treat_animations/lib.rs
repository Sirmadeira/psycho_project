use bevy::prelude::*;
use bevy::time::Timer;
use bevy::utils::Duration;

// Tells me which type of movement i should pass, to avoid multiple arguments or enums
#[derive(Event, Clone, Copy, Debug)]
pub enum AnimationType {
    None,
    Idle,
    FrontWalk,
    BackWalk,
    LeftWalk,
    RightWalk,
    FrontDash,
    LeftDash,
}
pub struct AnimationProperties {
    pub name: &'static str,
    pub duration: Duration,
    pub repeat: bool,
    pub cooldown: Option<Duration>,
}

impl AnimationProperties {
    pub fn new(
        name: &'static str,
        duration: Duration,
        repeat: bool,
        cooldown: Option<Duration>,
    ) -> Self {
        Self {
            name,
            duration,
            repeat,
            cooldown,
        }
    }
}

impl AnimationType {
    pub fn properties(self) -> AnimationProperties {
        match self {
            AnimationType::Idle => {
                AnimationProperties::new("Idle", Duration::from_millis(200), false, None)
            }
            AnimationType::FrontWalk => {
                AnimationProperties::new("FrontWalk", Duration::from_millis(200), true, None)
            }
            AnimationType::BackWalk => {
                AnimationProperties::new("BackWalk", Duration::from_millis(200), true, None)
            }
            AnimationType::LeftWalk => {
                AnimationProperties::new("LeftWalk", Duration::from_millis(200), true, None)
            }
            AnimationType::RightWalk => {
                AnimationProperties::new("RightWalk", Duration::from_millis(200), true, None)
            }
            AnimationType::FrontDash => {
                AnimationProperties::new("FrontDash", Duration::from_millis(200), false, None)
            }
            AnimationType::LeftDash => {
                AnimationProperties::new("LeftDash", Duration::from_millis(200), false, None)
            }

            AnimationType::None => AnimationProperties::new("None", Duration::ZERO, false, None),
        }
    }
}

#[derive(Reflect, Component, Debug)]
#[component(storage = "SparseSet")]
pub struct AnimationCooldown(pub Timer);
