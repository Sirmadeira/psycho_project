use bevy::prelude::*;
use bevy::utils::{Duration, HashMap};

//Animation
// This is a resource, that I am gonna use to play the pre imported clips
#[derive(Resource, Reflect)]
pub struct Animations {
    pub named_nodes: HashMap<String, AnimationNodeIndex>,
    pub animation_graph: Handle<AnimationGraph>,
}

// This one plays the blended clips or masked and so on.
#[derive(Resource, Reflect)]
pub struct BlendAnimations {
    pub node: Vec<AnimationNodeIndex>,
    pub animation_graph: Handle<AnimationGraph>,
}

// Marker component serves to point out the unique animated entity of player
#[derive(Reflect, Component, Debug)]
pub struct AnimatedEntity;

// Deine which animations to blend together, just add more here if you want more bone masked animations
#[derive(Resource)]
pub struct ConfigBoneMaskedAnimations(pub Vec<MaskNode>);

impl Default for ConfigBoneMaskedAnimations {
    fn default() -> Self {
        let mut vec = Vec::new();

        let first_mask = MaskNode {
            first_anim: "FrontWalk".to_string(),
            second_anim: "FrontAttack".to_string(),
            first_anim_clip: None,
            second_anim_clip: None,
        };

        vec.push(first_mask);

        ConfigBoneMaskedAnimations(vec)
    }
}

// Config tell me which animation clips to blend and so on
pub struct MaskNode {
    pub first_anim: String,
    pub second_anim: String,
    pub first_anim_clip: Option<Handle<AnimationClip>>,
    pub second_anim_clip: Option<Handle<AnimationClip>>,
}

// Marker component tells me which bones to override
#[derive(Component, Debug)]
pub struct BoneMask;

// Tells me which type of movement i should pass, to avoid multiple arguments or enums
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
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
    pub fn new(name: &'static str, duration: Duration, repeat: bool) -> Self {
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
                AnimationProperties::new("Idle", Duration::from_millis(400), false)
            }
            AnimationType::FrontWalk => {
                AnimationProperties::new("FrontWalk", Duration::from_millis(400), true)
            }
            AnimationType::BackWalk => {
                AnimationProperties::new("BackWalk", Duration::from_millis(400), true)
            }
            AnimationType::LeftWalk => {
                AnimationProperties::new("LeftWalk", Duration::from_millis(400), true)
            }
            AnimationType::RightWalk => {
                AnimationProperties::new("RightWalk", Duration::from_millis(400), true)
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
            AnimationType::Landing => {
                AnimationProperties::new("Landing", Duration::from_millis(0), false)
            }
            AnimationType::None => AnimationProperties::new("None", Duration::ZERO, false),
        }
    }
}
