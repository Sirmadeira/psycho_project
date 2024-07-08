use crate::mod_char_plugin::helpers::{collect_bones, find_child_with_name_containing};
use bevy::prelude::*;
use bevy::utils::HashMap;

// Will grab the main skeleton entity
pub fn get_main_skeleton_bones_and_armature(
    children_entities: &Query<&Children>,
    names: &Query<&Name>,
    skeleton_entity_id: &Entity,
) -> HashMap<String, Entity> {
    let mut main_bones = HashMap::new();

    let root_bone = find_child_with_name_containing(children_entities, &names, &skeleton_entity_id, "Hips").expect("the skeleton to have a bone called 'Root'");

    collect_bones(children_entities, &names, &root_bone, &mut main_bones);

    return main_bones;
}

pub fn attach_part_to_main_skeleton(
    commands: &mut Commands,
    children_entities: &Query<&Children>,
    names: &Query<&Name>,
    part_scene_entity: &Entity,
    main_skeleton_bones: &HashMap<String, Entity>,
) {
    let root_bone_option =
        find_child_with_name_containing(children_entities, names, &part_scene_entity, "Hips");

    if let Some(root_bone) = root_bone_option {
        let mut part_bones = HashMap::new();
        collect_bones(children_entities, names, &root_bone, &mut part_bones);
        for (name, part_bone) in part_bones {
            let mut entity_commands = commands.entity(part_bone);
            let new_parent_option = main_skeleton_bones.get(&name);

            if let Some(new_parent) = new_parent_option {
                entity_commands.set_parent_in_place(*new_parent);
            }
        }
    }
}
