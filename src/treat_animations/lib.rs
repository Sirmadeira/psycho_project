use bevy::prelude::*;
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
    RightDash,
    BackDash,
    Jump,
    FrontAir,
    BackAir,
    LeftAir,
    RightAir,
    Landing,
}

pub struct AnimationProperties {
    pub name: &'static str,
    pub duration: Duration,
    pub repeat: bool,
}

impl AnimationProperties {
    pub fn new(
        name: &'static str,
        duration: Duration,
        repeat: bool,
    ) -> Self {
        Self {
            name,
            duration,
            repeat,
        }
    }
}

impl AnimationType {
    pub fn properties(self) -> AnimationProperties {
        match self {
            AnimationType::Idle => {
                AnimationProperties::new("Idle", Duration::from_millis(200), false)
            }
            AnimationType::FrontWalk => {
                AnimationProperties::new("FrontWalk", Duration::from_millis(200), true)
            }
            AnimationType::BackWalk => {
                AnimationProperties::new("BackWalk", Duration::from_millis(200), true)
            }
            AnimationType::LeftWalk => {
                AnimationProperties::new("LeftWalk", Duration::from_millis(200), true)
            }
            AnimationType::RightWalk => {
                AnimationProperties::new("RightWalk", Duration::from_millis(200), true)
            }
            AnimationType::FrontDash => {
                AnimationProperties::new("FrontDash", Duration::from_millis(0), false)
            }
            AnimationType::LeftDash => {
                AnimationProperties::new("LeftDash", Duration::from_millis(0), false)
            }
            AnimationType::RightDash => {
                AnimationProperties::new("RightDash", Duration::from_millis(0), false)
            }
            AnimationType::BackDash => {
                AnimationProperties::new("BackDash", Duration::from_millis(0), false)
            }
            AnimationType::Jump => {
                AnimationProperties::new("Jump", Duration::from_millis(0), false)
            }
            AnimationType::FrontAir => {
                AnimationProperties::new("FrontAir", Duration::from_millis(400), false)
            }
            AnimationType::BackAir => {
                AnimationProperties::new("BackAir", Duration::from_millis(400), false)
            }
            AnimationType::LeftAir => {
                AnimationProperties::new("LeftAir", Duration::from_millis(500), false)
            }
            AnimationType::RightAir => {
                AnimationProperties::new("RightAir", Duration::from_millis(500), false)
            }
            AnimationType::Landing => AnimationProperties::new(
                "Landing",
                Duration::from_millis(0),
                false,
            ),
            AnimationType::None => AnimationProperties::new("None", Duration::ZERO, false),
        }
    }
}
