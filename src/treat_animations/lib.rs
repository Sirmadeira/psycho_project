

use bevy::prelude::*;
use bevy::utils::HashMap;

// This is a resource, that I am gonna use to have easy acess to the info of my animation graphs
#[derive(Resource, Reflect)]
pub struct Animations {
    pub named_nodes: HashMap<String, AnimationNodeIndex>,
    pub animation_graph: Handle<AnimationGraph>,
}

// Tells me which type of movement i should pass, to avoid multiple arguments or enums
#[derive(Event, Debug)]
pub enum AnimationType {
    // If it is forward backwards and so on
    None,
    FrontWalk,
    LeftWalk
}

