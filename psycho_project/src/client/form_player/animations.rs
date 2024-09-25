//! Player related animations are here
use bevy::prelude::*;
use bevy::utils::HashMap;
use lightyear::prelude::Replicated;

use crate::client::load_assets::ClientCharCollection;
use crate::shared::protocol::player_structs::PlayerVisuals;

use crate::client::form_player::is_loaded;
pub struct AnimPlayer;

impl Plugin for AnimPlayer {
    fn build(&self, app: &mut App) {
        app.register_type::<Animations>();
        app.register_type::<PlayerAnimationMap>();
        app.add_systems(Startup, create_animations_resource);
        // In this state to avoid running animations when character still transfering animations
        // app.add_systems(OnEnter(MyCharState::Done), mark_player_animation_players);
        app.add_systems(Update, create_anim_transitions);
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

// Resource utilized to easily find the animation players that should be played
// Pass player entity, get vec of entities that have animation players
#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct PlayerAnimationMap(HashMap<Entity, Vec<Entity>>);

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

// Inserts in every enttiy with an animation player an animation transition
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

// THis function maps the animation players and their main parent entities
fn mark_player_animation_players() {}

// Grabbing animations from gltf and inserting into graph
fn insert_gltf_animations(
    player_visuals: Query<&PlayerVisuals, Added<Replicated>>,
    char_collection: Res<ClientCharCollection>,
    assets_gltf: Res<Assets<Gltf>>,
    mut animations: ResMut<Animations>,
    mut assets_animation_graph: ResMut<Assets<AnimationGraph>>,
    
) {
    info_once!("Gettting handle for animation graph");
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
            // info!(
            //     "Current available animations are {} for skeleton {}",
            //     name_animation, skeleton
            // );
        }
    }
}

fn play_animation(animation_entities: Query<Entity, With<AnimationPlayer>>) {}
