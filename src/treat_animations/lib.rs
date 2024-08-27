use crate::player_mechanics::lib::ActionProperties;
use bevy::prelude::*;
use bevy::utils::HashMap;

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
        let walk_anims = vec!["FrontWalk", "BackWalk", "LeftWalk", "RightWalk", "Idle"];
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
#[derive(Event)]
pub struct AnimationType(pub ActionProperties);
