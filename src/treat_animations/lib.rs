use bevy::prelude::*;
use bevy::utils::{Duration, HashMap};

//Animation
// This is a resource, that I am gonna use to play the pre imported clips
#[derive(Resource, Reflect)]
pub struct Animations {
    pub named_nodes: HashMap<String, AnimationNodeIndex>,
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
        // Define the "walk" and "attack" animations
        let walk_anims = vec!["FrontWalk", "BackWalk", "LeftWalk", "RightWalk"];
        let attack_anims = vec!["FrontAttack", "BackAttack", "LeftAttack", "RightAttack"];
        
        // Create a mutable vector to hold all combinations
        let mut vec = Vec::new();
        
        // Iterate over all combinations of "walk" and "attack" animations
        for walk in &walk_anims {
            for attack in &attack_anims {
                let mask_node = MaskNode {
                    first_anim: walk.to_string(),
                    second_anim: attack.to_string(),
                    first_anim_clip: None,
                    second_anim_clip: None,
                };
                vec.push(mask_node);
            }
        }
        
        // Create and return the ConfigBoneMaskedAnimations with all combinations
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
#[derive(Event, Clone,Debug, PartialEq, Eq)]
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
    BlendAnimation(String),
}

pub struct AnimationProperties {
    pub name: String,
    pub duration: Duration,
    pub repeat: bool,
}

impl AnimationProperties {
    pub fn new(name: String, duration: Duration, repeat: bool) -> Self {
        Self {
            name,
            duration,
            repeat,
        }
    }
}


impl AnimationType {
    pub fn properties(&self) -> AnimationProperties {
        match self {
            AnimationType::Idle => {
                AnimationProperties::new("Idle".to_string(), Duration::from_millis(400), false)
            }
            AnimationType::FrontWalk => {
                AnimationProperties::new("FrontWalk".to_string(), Duration::from_millis(400), true)
            }
            AnimationType::BackWalk => {
                AnimationProperties::new("BackWalk".to_string(), Duration::from_millis(400), true)
            }
            AnimationType::LeftWalk => {
                AnimationProperties::new("LeftWalk".to_string(), Duration::from_millis(400), true)
            }
            AnimationType::RightWalk => {
                AnimationProperties::new("RightWalk".to_string(), Duration::from_millis(400), true)
            }
            AnimationType::FrontDash => {
                AnimationProperties::new("FrontDash".to_string(), Duration::from_millis(0), false)
            }
            AnimationType::LeftDash => {
                AnimationProperties::new("LeftDash".to_string(), Duration::from_millis(0), false)
            }
            AnimationType::RightDash => {
                AnimationProperties::new("RightDash".to_string(), Duration::from_millis(0), false)
            }
            AnimationType::BackDash => {
                AnimationProperties::new("BackDash".to_string(), Duration::from_millis(0), false)
            }
            AnimationType::Jump => {
                AnimationProperties::new("Jump".to_string(), Duration::from_millis(0), false)
            }
            AnimationType::FrontAir => {
                AnimationProperties::new("FrontAir".to_string(), Duration::from_millis(400), false)
            }
            AnimationType::BackAir => {
                AnimationProperties::new("BackAir".to_string(), Duration::from_millis(400), false)
            }
            AnimationType::LeftAir => {
                AnimationProperties::new("LeftAir".to_string(), Duration::from_millis(500), false)
            }
            AnimationType::RightAir => {
                AnimationProperties::new("RightAir".to_string(), Duration::from_millis(500), false)
            }
            AnimationType::Landing => {
                AnimationProperties::new("Landing".to_string(), Duration::from_millis(0), false)
            }
            AnimationType::BlendAnimation(name) => {
                AnimationProperties::new(name.to_string(), Duration::from_millis(0), false)
            }
            AnimationType::None=> {
                AnimationProperties::new("NOne".to_string(), Duration::from_millis(0), false)
            }
        }
    }
}

