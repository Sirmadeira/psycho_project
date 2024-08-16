use bevy::{prelude::*, time::Stopwatch, utils::HashMap};


// Mod char
// Tell me quantity of players
#[derive(Resource, Reflect)]
pub struct AmountPlayers {
    pub quantity: u32,
}
// Resource that defines what to add to skeleton - TODO make UI that definesi
#[derive(Resource, Reflect)]
pub struct ConfigModularCharacters {
    pub visuals_to_be_attached: Vec<String>,
    pub weapons_to_be_attached: Vec<String>,
}

// Points out visual entities and weapon entities
#[derive(Component, Reflect)]
pub struct Attachments {
    pub visual: Vec<Option<Entity>>,
    pub weapons: Vec<Option<Entity>>,
}

// Simple marker components that points out entities that can become the player
#[derive(Component)]
pub struct Skeleton;

// Marker compoenent for visual
#[derive(Component)]
pub struct Visual;

// Tell me in which state the scene is
#[derive(States, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub enum StateSpawnScene {
    #[default]
    Spawning,
    Spawned,
    Done,
}

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

#[derive(Component, Reflect, Debug)]
pub struct Health(pub i8);

// Kind of a simple pid
#[derive(Reflect, Component, Debug)]
pub struct PdInfo {
    pub kp: f32,
}

// Times the dash for each key
#[derive(Reflect, Component, Debug)]
pub struct Timers {
    pub up: Stopwatch,
    pub down: Stopwatch,
    pub left: Stopwatch,
    pub right: Stopwatch,
}

// Marker component - Tells me which is the collider to check for ground
#[derive(Component)]
pub struct PlayerGroundCollider;


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
