
use bevy::prelude::*;

use bevy::utils::{Duration,HashMap};
use crate::treat_animations::lib::*;
use crate::load_assets_plugin::MyAssets;
use crate::mod_char::lib::AmountPlayers;
use crate::player_effects::lib::Player;


// Creates animation graph for each player and add it is clips to it
pub fn spawn_animations_graphs(
    amount_players: Res<AmountPlayers>,
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut assets_animation_graph: ResMut<Assets<AnimationGraph>>,
    mut commands: Commands,
) {
    for number_of_player in 1..=amount_players.quantity {
        // Creating graphs according to amount of player
        let mut graph = AnimationGraph::new();

        // Node with a string name
        let mut named_nodes = HashMap::new();

        // Using bevy asset loader to easily acess my assets
        for (_, gltf_handle) in &asset_pack.gltf_files {
            let gltf = assets_gltf
                .get(gltf_handle)
                .expect("My asset pack to have GLTF");

            // Creating named nodes
            for (name_animation, animation_clip) in gltf.named_animations.iter() {
                // Returns animations node
                let node = graph.add_clip(animation_clip.clone(), 1.0, graph.root);
                // Creating named node
                named_nodes.insert(name_animation.to_string(), node);
                println!(
                    "Current available animations are {} for player {}",
                    name_animation, number_of_player
                );
            }
        }

        // Adding animation graph to assets
        let anim_graph = assets_animation_graph.add(graph);

        // Formulating resource that tells me what is the name of the animation in a node and it is animation graph
        commands.insert_resource(Animations {
            named_nodes: named_nodes,
            animation_graph: anim_graph.clone(),
        });
    }
}



// Loads from assets and put into our animations players must have for animation playing
pub fn add_animation_graph(
    mut commands: Commands,
    animations: Res<Animations>,
    mut players: Query<Entity, Added<AnimationPlayer>>,
) {
    // Each skinned mesh already  comes with a prespawned animation player struct
    for entity  in &mut players {
        commands
            .entity(entity)
            .insert(animations.animation_graph.clone());
    }
}


// This will handle animation according to input events given by player_effects or other plugins
pub fn state_machine(
    player_skeleton: Query<&Player, With<Player>>,
    mut animation_to_play: EventReader<AnimationType>,

    mut components: Query<(&mut AnimationPlayer, Option< &mut AnimationTransitions>)>,
    animations: Res<Animations>,
    mut commands: Commands
) {


    let entity = player_skeleton.get_single().expect("To have skeleton").0;

    let (mut animation_player,active_transitions)  = components.get_mut(entity).expect("Skeleton components");

    if let Some(mut active_transition) = active_transitions{
        for event in animation_to_play.read(){

            match event {
                AnimationType::FrontWalk =>{
                    let animation = animations.named_nodes["FrontWalk"];
                    active_transition.play(&mut animation_player,animation , Duration::from_secs(1));
                }
                AnimationType::LeftWalk =>{
    
                }
                AnimationType::None =>{
                    // let animation = animations.named_nodes["Idle"];
                    // active_transition.play(&mut animation_player,animation , Duration::from_secs(10));
                }
    
            }
        }
    }
    // Treats the scenario where the  is no animation transition in main player
    else{
        let mut  transitions = AnimationTransitions::new();

        transitions.play(&mut animation_player, animations.named_nodes["Idle"], Duration::ZERO).repeat();
        
        commands.entity(entity).insert(transitions);


    }


}