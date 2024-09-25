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
        app.add_systems(Startup, create_animations_resource);
        app.add_systems(Update, insert_gltf_animations.run_if(is_loaded));
    }
}

//Resource utilized to tell me what animation to play in my animation graph
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct Animations {
    // A node  that tells me exactly the name of an specific animation
    pub named_nodes: HashMap<String, AnimationNodeIndex>,
    // It is graph handle
    pub animation_graph: Handle<AnimationGraph>,
}

fn create_animations_resource(
    mut assets_animation_graph: ResMut<Assets<AnimationGraph>>,
    mut commands: Commands,
) {
    let animation_named_nodes: HashMap<String, AnimationNodeIndex> = HashMap::default();
    let animation_graph = AnimationGraph::default();

    let hand_graph = assets_animation_graph.add(animation_graph.clone());

    commands.insert_resource(Animations {
        named_nodes: animation_named_nodes,
        animation_graph: hand_graph,
    });
}

// Grabbing animations from gltf and inserting into graph
fn insert_gltf_animations(
    player_visuals: Query<&PlayerVisuals, Added<Replicated>>,
    char_collection: Res<ClientCharCollection>,
    assets_gltf: Res<Assets<Gltf>>,
    mut animations: ResMut<Animations>,
    mut assets_animation_graph: ResMut<Assets<AnimationGraph>>,
) {
    info!("Gettting handle for animation graph");
    let animation_graph = assets_animation_graph
        .get_mut(&animations.animation_graph)
        .expect("To have created animation graph");

    for player_visual in player_visuals.iter() {
        //Skeleton should be the entity to carry all animations
        let skeleton = &player_visual.skeleton;

        let skeleton_gltf = char_collection
            .gltf_files
            .get(skeleton)
            .expect("To find skeleton in client collection");

        let gltf = assets_gltf
            .get(skeleton_gltf)
            .expect("Skeleton path to be found");

        for (name_animation, animation_clip) in gltf.named_animations.iter() {
            let node = animation_graph.add_clip(animation_clip.clone(), 1.0, animation_graph.root);
            animations
                .named_nodes
                .insert(name_animation.to_string(), node);
            info!(
                "Current available animations are {} for skeleton {}",
                name_animation, skeleton
            );
        }
    }
}
