use crate::mod_char_plugin::{
    link_animations::AnimationEntityLink,
    spawn_scenes::{Animations, SceneEntitiesByName, SceneName},
};
use bevy::prelude::*;

pub fn run_animations(
    mut animation_player_query: Query<&mut AnimationPlayer>,
    scene_and_animation_player_link_query: Query<
        (&SceneName, &AnimationEntityLink),
        Added<AnimationEntityLink>,
    >,
    animations: Res<Animations>,
    scene_entities_by_name: Res<SceneEntitiesByName>,
) {
    let main_skeleton_scene_entity = scene_entities_by_name
        .0
        .get("skeleton.glb")
        .expect("The scene to be named and created");

    let (_, animation_player_entity_link) = scene_and_animation_player_link_query
        .get(*main_skeleton_scene_entity)
        .expect("The scene skeleton to  have an animation");

    let mut animation_player = animation_player_query
        .get_mut(animation_player_entity_link.0)
        .expect("To have an animation player on the main skeleton");

    // Make this an event reader
    animation_player
        .play(
            animations
                .0
                .get("Sword_Slash")
                .expect("To have animation with this name to check available names look at spawn_scenes")
                .clone_weak(),
        )
        .repeat()
        .set_speed(1.0);
}
