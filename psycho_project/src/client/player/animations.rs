//! Player related animations are here
use crate::client::load_assets::CharCollection;
use crate::shared::protocol::player_structs::*;
use bevy::prelude::*;
use bevy::utils::{Duration, HashMap};
use leafwing_input_manager::prelude::ActionState;
use lightyear::prelude::client::Predicted;

use crate::client::MyAppState;

pub struct AnimPlayerPlugin;

impl Plugin for AnimPlayerPlugin {
    fn build(&self, app: &mut App) {
        // Debuggin
        app.register_type::<Animations>();
        app.register_type::<PointerAnimatedEntities>();

        // Systems
        app.add_systems(Startup, create_animations_resource);

        app.add_systems(OnEnter(MyAppState::MainMenu), insert_gltf_animations);

        app.add_systems(Update, (add_anim_components_to_player, form_pointer));

        app.add_systems(
            Update,
            (create_anim_transitions, add_animation_graph).chain(),
        );
        //IMPORTANT ONLY PLAY ANIMATION AFTER ADDING ANIMATION GRAPH
        app.add_systems(Update, state_machine.after(add_animation_graph));
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

/// A usefull component inserted in predicted player entities that tell me exactly who are the children with animation players
/// Utilized mostly for optimizations
#[derive(Component, Default, Reflect)]
struct PointerAnimatedEntities(Vec<Entity>);

/// Helper function gives me the parent of animation player entities
fn get_top_parent(mut curr_entity: Entity, entities_with_parent: &Query<&Parent>) -> Entity {
    //Loop up all the way to the top parent
    loop {
        if let Ok(ref_to_parent) = entities_with_parent.get(curr_entity) {
            curr_entity = ref_to_parent.get();
        } else {
            break;
        }
    }
    curr_entity
}

/// Essential components to have when player gets predicted
fn add_anim_components_to_player(
    query: Query<Entity, (Added<Predicted>, With<PlayerId>)>,
    mut commands: Commands,
) {
    for predicted in query.iter() {
        commands
            .entity(predicted)
            .insert(PointerAnimatedEntities::default());
    }
}

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
    for entity in animated_entities.iter() {
        // info!("Inserting anim transition into entity {}", entity);
        let transitions = AnimationTransitions::new();
        commands.entity(entity).insert(transitions);
    }
}

/// Important this is based on the fact that I always set player as parent of animated scenes
/// If by any means the logic changes this will break
fn form_pointer(
    animated_entities: Query<Entity, Added<AnimationPlayer>>,
    entities_with_parent: Query<&Parent>,
    mut pointer: Query<&mut PointerAnimatedEntities>,
) {
    for entity in animated_entities.iter() {
        // Ideally this should be player all the time
        let player = get_top_parent(entity, &entities_with_parent);

        if let Ok(mut pointer) = pointer.get_mut(player) {
            // info!(
            //     "Manage to acess player pointer to animation player {}",
            //     player
            // );
            pointer.0.push(entity);
        } else {
            warn!(
                "Didnt manage to make a pointer to the following animated entity {}",
                entity
            );
        }
    }
}

/// Loads from assets and put into our animations players must have for animation playing
fn add_animation_graph(
    animations: Res<Animations>,
    mut commands: Commands,
    mut players: Query<Entity, Added<AnimationPlayer>>,
) {
    // Each skinned mesh already  comes with a prespawned animation player struct
    for entity in &mut players {
        // info!("Inserting anim graph into entity {}", entity);
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

/// Since it already passes this information no need in input channel I dont need to set it
fn state_machine(
    query: Query<(&ActionState<PlayerAction>, &PointerAnimatedEntities), With<Predicted>>,
    mut animation_components: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    animations: Res<Animations>,
) {
    let named_animations = animations.named_nodes.clone();

    for (action_state, pointer_animated) in query.iter() {
        for animated_entity in pointer_animated.0.iter() {
            if let Ok((mut animation_player, mut animation_transition)) =
                animation_components.get_mut(*animated_entity)
            {
                if action_state.just_pressed(&PlayerAction::Jump) {
                    let node = named_animations.get("Sword_Slash").unwrap();
                    animation_transition.play(&mut animation_player, *node, Duration::ZERO);
                }
            }
        }
    }
}
