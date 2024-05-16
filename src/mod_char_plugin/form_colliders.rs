use super::link_animations::AnimationEntityLink;
use crate::mod_char_plugin::assemble_parts::find_child_with_name_containing::find_child_with_name_containing;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// Find the torso bone. Insert a collider on it as we dont really want many colliders I am gonna make it semi-manually
pub fn insert_colliders(
    mut commands:  Commands,
    transforms: Query<&Transform>,
    all_entities_with_children: Query<&Children>,
    main_entity_option: Query<Entity, With<AnimationEntityLink>>,
    names: Query<&Name>,
) {
    let Ok(main_entity) = main_entity_option.get_single() else {
        println!("No player entity available");
        return;
    };

    let torso_entity_option =
        find_child_with_name_containing(&all_entities_with_children, &names, &main_entity, "Torso");

    if let Some(torso_entity) = torso_entity_option {
        let torso_transform = transforms.get(torso_entity);
        let torso = (RigidBody::Fixed, Collider::ball(0.5));
        // TODO INTERPOLATION
        commands.entity(torso_entity).insert(torso);
    }
}
