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
    RightDigWalk,
    BackRightDigWalk,
    LeftDigWalk,
    BackLeftDigWalk,
}
pub struct AnimationProperties {
    pub name: &'static str,
    pub duration: Duration,
    pub repeat: bool,
    pub cooldown: Option<Duration>,
    pub after_anim: Option<&'static str>,
}

impl AnimationProperties {
    pub fn new(
        name: &'static str,
        duration: Duration,
        repeat: bool,
        cooldown: Option<Duration>,
        after_anim: Option<&'static str>,
    ) -> Self {
        Self {
            name,
            duration,
            repeat,
            cooldown,
            after_anim,
        }
    }
}

impl AnimationType {
    pub fn properties(self) -> AnimationProperties {
        match self {
            AnimationType::Idle => {
                AnimationProperties::new("Idle", Duration::from_millis(200), false, None, None)
            }
            AnimationType::FrontWalk => {
                AnimationProperties::new("FrontWalk", Duration::from_millis(200), true, None, None)
            }
            AnimationType::BackWalk => {
                AnimationProperties::new("BackWalk", Duration::from_millis(200), true, None, None)
            }
            AnimationType::LeftWalk => {
                AnimationProperties::new("LeftWalk", Duration::from_millis(200), true, None, None)
            }
            AnimationType::RightWalk => {
                AnimationProperties::new("RightWalk", Duration::from_millis(200), true, None, None)
            }
            AnimationType::RightDigWalk => AnimationProperties::new(
                "RightDigWalk",
                Duration::from_millis(200),
                false,
                None,
                Some("FrontWalk"),
            ),
            AnimationType::BackRightDigWalk => AnimationProperties::new(
                "BackRightDigWalk",
                Duration::from_millis(200),
                false,
                None,
                Some("BackWalk"),
            ),
            AnimationType::LeftDigWalk => AnimationProperties::new(
                "LeftDigWalk",
                Duration::from_millis(200),
                false,
                None,
                Some("FrontWalk"),
            ),
            AnimationType::BackLeftDigWalk => AnimationProperties::new(
                "BackLeftDigWalk",
                Duration::from_millis(200),
                false,
                None,
                Some("BackWalk"),
            ),
            AnimationType::None => {
                AnimationProperties::new("None", Duration::ZERO, false, None, None)
            }
        }
    }
}

#[derive(Reflect, Component, Debug)]
#[component(storage = "SparseSet")]
pub struct AnimationCooldown(pub Timer);

#[derive(Reflect, Component, Debug)]
#[component(storage = "SparseSet")]
pub struct DiagonalAnimation;

#[derive(Event, Clone, Copy, Debug)]
pub struct AfterAnim(pub &'static str);

impl AfterAnim {
    pub fn properties(&self) -> AnimationProperties {
        match self.0 {
            "FrontWalk" => {
                AnimationProperties::new("FrontWalk", Duration::from_millis(200), true, None, None)
            }
            "BackWalk" => {
                AnimationProperties::new("BackWalk", Duration::from_millis(200), true, None, None)
            }
            _ => todo!(),
        }
    }
}

// Marker component serves to point out the unique animated entity of player
#[derive(Reflect, Component, Debug)]
pub struct AnimatedEntity;