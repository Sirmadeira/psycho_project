//! Player related animations are here

use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::client::load_assets::CharCollection;

use crate::client::MyAppState;

pub struct AnimPlayerPlugin;

impl Plugin for AnimPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Animations>();
        app.register_type::<PlayerAnimationMap>();
        app.add_systems(Startup, create_animations_resource);
        app.add_systems(Update, (create_anim_transitions, add_animation_graph));
        app.add_systems(OnEnter(MyAppState::MainMenu), insert_gltf_animations);
    }
}

/// Resource utilized to tell me what animation to play in my animation graph
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct Animations {
    // A node  that tells me exactly the name of an specific animation
    pub named_nodes: HashMap<String, AnimationNodeIndex>,
    // It is graph handle
    pub animation_graph: Handle<AnimationGraph>,
}

/// Resource utilized to easily find the animation players that should be played
/// Pass player entity, get vec of entities that have animation players
#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct PlayerAnimationMap(HashMap<Entity, Vec<Entity>>);

/// Creates animations a resource that basically stores in a node every single named animation our gltf files may have
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

/// Inserts in every enttiy with an animation player an animation transition
fn create_anim_transitions(
    animated_entities: Query<Entity, Added<AnimationPlayer>>,
    mut commands: Commands,
) {
    info_once!("Inserting anim transition");
    for entity in animated_entities.iter() {
        let transitions = AnimationTransitions::new();
        commands.entity(entity).insert(transitions);
    }
}

/// Loads from assets and put into our animations players must have for animation playing
pub fn add_animation_graph(
    animations: Res<Animations>,
    mut commands: Commands,
    mut players: Query<Entity, Added<AnimationPlayer>>,
) {
    // Each skinned mesh already  comes with a prespawned animation player struct
    for entity in &mut players {
        commands
            .entity(entity)
            .insert(animations.animation_graph.clone());
    }
}

/// Grabbing animations from gltf and inserting into graph - TODO EXPAND THIS TO GRAB ALL SKELETONS
fn insert_gltf_animations(
    char_collection: Res<CharCollection>,
    assets_gltf: Res<Assets<Gltf>>,
    mut animations: ResMut<Animations>,
    mut assets_animation_graph: ResMut<Assets<AnimationGraph>>,
) {
    info_once!("Gettting handle for animation graph");
    let animation_graph = assets_animation_graph
        .get_mut(&animations.animation_graph)
        .expect("To have created animation graph");

    let skeleton_gltf = char_collection
        .gltf_files
        .get("characters/parts/main_skeleton.glb")
        .expect("To find skeleton in client collection");

    let gltf = assets_gltf
        .get(skeleton_gltf)
        .expect("Skeleton path to be found");

    for (name_animation, animation_clip) in gltf.named_animations.iter() {
        let node = animation_graph.add_clip(animation_clip.clone(), 1.0, animation_graph.root);
        animations
            .named_nodes
            .insert(name_animation.to_string(), node);
        // info!(
        //     "Current available animations are {} for skeleton {}",
        //     name_animation, "characters/mod_char/main_skeleton.glb"
        // );
    }
}

// fn play_animation(
//     mut animation_entities: Query<
//         (&mut AnimationTransitions, &mut AnimationPlayer),
//         Added<AnimationPlayer>,
//     >,
//     animations: Res<Animations>,
// ) {
//     let named_animations = animations.named_nodes.clone();
//     for (mut animation_transitions, mut animation_player) in animation_entities.iter_mut() {
//         let node = named_animations.get("Walk").unwrap();
//         animation_transitions
//             .play(&mut animation_player, *node, Duration::ZERO)
//             .repeat();
//     }
// }
