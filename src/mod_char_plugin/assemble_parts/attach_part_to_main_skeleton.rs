use bevy::{prelude::*, utils::HashMap};

use crate::mod_char_plugin::assemble_parts::{
    collect_bones::collect_bones, find_child_with_name_containing::find_child_with_name_containing,
};

// The logic here is that we grab the main skeleton bone and attach the bone from the module into it
// Making it is children, that is why we need to reset the transform, to ensure it stays in the same position

pub fn attach_part_to_main_skeleton(
    commands: &mut Commands,
    all_entities_with_children: &Query<&Children>,
    names: &Query<&Name>,
    part_scene_name: &String,
    part_scene_entity: &Entity,
    main_armature_entity: &Entity,
    main_skeleton_bones: &HashMap<String, Entity>,
) {
    println!("Attaching part: {}", part_scene_name);

    let root_bone_option = find_child_with_name_containing(
        all_entities_with_children,
        names,
        &part_scene_entity,
        "Root",
    );

    let part_armature_option = find_child_with_name_containing(
        all_entities_with_children,
        names,
        &part_scene_entity,
        "CharacterArmature",
    );

    if let Some(part_armature) = part_armature_option {
        let mut part_armature_entity_commands = commands.entity(part_armature);
        part_armature_entity_commands.set_parent_in_place(*main_armature_entity);
        // DO NOT DESPAWN AS THIS HOLDS THE DATA TO THE MESHES somehow
        // Interestingly if you delete it the meshes still appear
        // But if you hide them they dont
    }

    // FOR SOME UNGODLY REASON THE FUCKING BONE WITH NO ANIMATION DATA STAYS WITH THE ANIMATION DATA THEREFORE I CANT DELETE IT
    if let Some(root_bone) = root_bone_option {
        let mut part_bones = HashMap::new();
        collect_bones(
            all_entities_with_children,
            names,
            &root_bone,
            &mut part_bones,
        );
        for (name, part_bone) in part_bones {
            println!("Attaching {}, {:#?}", name, part_bone);

            let mut entity_commands = commands.entity(part_bone);
            let new_parent_option = main_skeleton_bones.get(&name);

            if let Some(new_parent) = new_parent_option {
                entity_commands.set_parent_in_place(*new_parent);
            }
        }
    }
    // Despawn the gltfs we sucked dry
    commands.entity(*part_scene_entity).despawn();
}



fn make_colliders(){
    
}