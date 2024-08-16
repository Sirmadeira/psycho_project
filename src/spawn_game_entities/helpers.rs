use bevy::prelude::*;
use bevy::utils::HashMap;
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
