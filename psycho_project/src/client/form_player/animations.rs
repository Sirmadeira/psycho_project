//! Player related animations are here
use bevy::prelude::*;
use bevy::utils::HashMap;
use lightyear::prelude::Replicated;

use crate::client::load_assets::ClientCharCollection;
use crate::client::MyAppState;
use crate::shared::protocol::player_structs::PlayerVisuals;

use crate::client::form_player::is_loaded;
pub struct AnimPlayer;

impl Plugin for AnimPlayer {
    fn build(&self, app: &mut App) {
        app.register_type::<Animations>();
        app.add_systems(Update, grab_gltf_animations.run_if(is_loaded));
    }
}

//Resource utilized to tell me what animation to play in my animation graph
#[derive(Resource, Reflect)]
pub struct Animations {
    // A node  that tells me exactly the name of an specific animation
    pub named_nodes: HashMap<String, AnimationNodeIndex>,
    // It is graph handle
    pub animation_graph: Handle<AnimationGraph>,
}

// Grabbing animations from gltf
pub fn grab_gltf_animations(
    player_visual: Query<&PlayerVisuals, Added<Replicated>>,
    char_collection: Res<ClientCharCollection>,
    assets_gltf: Res<Assets<Gltf>>,
    mut animations: ResMut<Animations>,
    mut assets_animation_graph: ResMut<Assets<AnimationGraph>>,
) {
    info!("Gettting handle for animation graph");
    let animation_graph = assets_animation_graph
        .get_mut(&animations.animation_graph)
        .expect("To have created animation graph");

    info!("Grabbing all the animation available in our assets and registering in resource");
    for (_, gltf_handle) in &char_collection.gltf_files {
        let gltf = assets_gltf
            .get(gltf_handle)
            .expect("My asset pack to have GLTF");

        for (name_animation, animation_clip) in gltf.named_animations.iter() {
            let node = animation_graph.add_clip(animation_clip.clone(), 1.0, animation_graph.root);
            animations
                .named_nodes
                .insert(name_animation.to_string(), node);
        }
    }

    for (name, _) in animations.named_nodes.clone() {
        info!("Current available animations are {} for player", name);
    }
}
