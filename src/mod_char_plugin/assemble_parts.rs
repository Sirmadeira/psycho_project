use super::spawn_scenes::{SceneEntitiesByName, SceneName};
use crate::mod_char_plugin::StateSpawnScene;
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::collections::VecDeque;

// Main function will call everyone
pub fn assemble_parts(
    mut commands: Commands,
    all_entities_with_children: Query<&Children>,
    scene_query: Query<(Entity, &SceneName), With<SceneName>>,
    scene_entities_by_name: Res<SceneEntitiesByName>,
    names: Query<&Name>,
    mut next_state: ResMut<NextState<StateSpawnScene>>,
) {
    let (main_skeleton_bones, main_armature_entity) = get_main_skeleton_bones_and_armature(
        scene_entities_by_name,
        &all_entities_with_children,
        &names,
    );

    for (part_scene_entity, part_scene_name) in &scene_query {
        if part_scene_name.0 == "sword.glb" {
            let mut sword_entity_commands = commands.entity(part_scene_entity);
            if let Some(handle_bone) = main_skeleton_bones.get("EquipmentHandle.R") {
                sword_entity_commands.set_parent(*handle_bone);
            }
        } else if part_scene_name.0 != "skeleton.glb" {
            attach_part_to_main_skeleton(
                &mut commands,
                &all_entities_with_children,
                &names,
                &part_scene_name.0,
                &part_scene_entity,
                &main_armature_entity,
                &main_skeleton_bones,
            );
        }
    }
    next_state.set(StateSpawnScene::FormingPhysics);
}

// Will grab the main skeleton entity
pub fn get_main_skeleton_bones_and_armature(
    scene_entities_by_name: Res<SceneEntitiesByName>,
    all_entities_with_children: &Query<&Children>,
    names: &Query<&Name>,
) -> (HashMap<String, Entity>, Entity) {
    let mut main_bones = HashMap::new();

    let main_skeleton_entity = scene_entities_by_name
        .0
        .get("skeleton.glb")
        .expect("to have spawned the main skeleton scene");

    let root_bone = find_child_with_name_containing(
        all_entities_with_children,
        &names,
        &main_skeleton_entity,
        "Root",
    )
    .expect("the skeleton to have a bone called 'Root'");

    let main_skeleton_armature = find_child_with_name_containing(
        all_entities_with_children,
        &names,
        &main_skeleton_entity,
        "CharacterArmature",
    )
    .expect("the skeleton to have an armature");

    collect_bones(
        all_entities_with_children,
        &names,
        &root_bone,
        &mut main_bones,
    );

    println!("Bones in main skeleton: {:#?}", main_bones);

    return (main_bones, main_skeleton_armature);
}

// Collects a lot of subchild bones
pub fn collect_bones(
    all_entities_with_children: &Query<&Children>,
    names: &Query<&Name>,
    root_bone: &Entity,
    collected: &mut HashMap<String, Entity>,
) {
    if let Ok(name) = names.get(*root_bone) {
        collected.insert(format!("{}", name), *root_bone);

        if let Ok(children) = all_entities_with_children.get(*root_bone) {
            for child in children {
                collect_bones(all_entities_with_children, names, child, collected)
            }
        }
    }
}

// Finds a bone with a certain name
pub fn find_child_with_name_containing(
    all_entities_with_children: &Query<&Children>,
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

        let children_result = all_entities_with_children.get(*curr_entity);
        if let Ok(children) = children_result {
            for child in children {
                queue.push_back(child)
            }
        }
    }

    return None;
}


pub fn attach_part_to_main_skeleton(
    commands: &mut Commands,
    all_entities_with_children: &Query<&Children>,
    names: &Query<&Name>,
    part_scene_name: &String,
    part_scene_entity: &Entity,
    main_armature_entity: &Entity,
    main_skeleton_bones: &HashMap<String, Entity>,
) {
    println!("Attaching loaded_asset part: {}", part_scene_name);

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
    // The child bones only appear twice in the same ROW of nodes if the bone exists in both of them
    if let Some(root_bone) = root_bone_option {
        let mut part_bones = HashMap::new();
        collect_bones(
            all_entities_with_children,
            names,
            &root_bone,
            &mut part_bones,
        );
        for (name, part_bone) in part_bones {
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
