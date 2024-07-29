use crate::spawn_game_entities::lib::AmountPlayers;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier3d::prelude::*;
use std::collections::VecDeque;

// Collects a lot of subchild bones
pub fn collect_bones(
    children_entities: &Query<&Children>,
    names: &Query<&Name>,
    root_bone: &Entity,
    collected: &mut HashMap<String, Entity>,
) {
    if let Ok(name) = names.get(*root_bone) {
        collected.insert(format!("{}", name), *root_bone);

        if let Ok(children) = children_entities.get(*root_bone) {
            for child in children {
                collect_bones(children_entities, names, child, collected)
            }
        }
    }
}

// Finds a bone with a certain name
pub fn find_child_with_name_containing(
    children_entities: &Query<&Children>,
    names: &Query<&Name>,
    entity: &Entity,
    name_to_match: &str,
) -> Option<Entity> {
    let mut queue = VecDeque::new();
    queue.push_back(entity);

    while let Some(curr_entity) = queue.pop_front() {
        let name_result = names.get(*curr_entity);
        if let Ok(name) = name_result {
            if format!("{}", name).contains(name_to_match) {
                // found the named entity
                return Some(*curr_entity);
            }
        }

        let children_result = children_entities.get(*curr_entity);
        if let Ok(children) = children_result {
            for child in children {
                queue.push_back(child)
            }
        }
    }

    return None;
}

pub fn create_dynamic_collider_groups(
    player_amount: &Res<AmountPlayers>,
    collision_number: u32,
    base_group: Option<Group>,
) -> CollisionGroups {
    let membership_group;
    let mut filter_group;

    // Only for weapons, may extend later
    if let Some(base_group) = base_group {
        membership_group =
            Group::from_bits(player_amount.quantity + 1).expect("TO have at least a membership");
        filter_group = base_group;
    } else {
        membership_group =
            Group::from_bits(collision_number).expect("TO have at least a membership");
        filter_group = Group::empty();
    }

    for group in (1..=player_amount.quantity).rev() {
        let to_be_group = Group::from_bits(group).expect("Group");
        if membership_group.ne(&to_be_group) {
            filter_group = filter_group | to_be_group;
        }
    }

    return CollisionGroups::new(membership_group, filter_group);
}
