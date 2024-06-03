use bevy::prelude::*;
use super::spawn_scenes::StateSpawnScene;


// This exists because bevy is very annoying with it is api
// And doesnt let me dissociate characters easily

#[derive(Component, Debug)]
pub struct AnimationEntityLink(pub Entity);

// Recursive function interacts through the hierarchy and grab the "first" parent of the scene
pub fn get_top_parent(
    mut curr_entity: Entity,
    all_entities_with_parents_query: &Query<&Parent>,
) -> Entity {
    loop {
        if let Ok(ref_to_parent) = all_entities_with_parents_query.get(curr_entity) {
            curr_entity = ref_to_parent.get();
        } else {
            break;
        }
    }
    curr_entity
}

// Put animation link in parent, that way avoid too many animation players query
pub fn link_animations(
    animation_players_query: Query<Entity, Added<AnimationPlayer>>,
    all_entities_with_parents_query: Query<&Parent>,
    animations_entity_link_query: Query<&AnimationEntityLink>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<StateSpawnScene>>,
) {
    // Get all the Animation players which can be deep and hidden in the heirachy
    for entity_with_animation_player in animation_players_query.iter() {
        let top_entity = get_top_parent(
            entity_with_animation_player,
            &all_entities_with_parents_query,
        );

        // If the top parent has an animation config ref then link the player to the config
        if animations_entity_link_query.get(top_entity).is_ok() {
            warn!("Problem with multiple animation players for the same top parent");
        } else {
            println!(
                "linking entity {:#?} to animation_player entity {:#?}",
                top_entity, entity_with_animation_player
            );
            commands
                .entity(top_entity)
                .insert(AnimationEntityLink(entity_with_animation_player.clone()));
        }
    }
    // Notice that the spawn scene will be done only after we add the animation to it
    next_state.set(StateSpawnScene::HandlingModularity)
}
