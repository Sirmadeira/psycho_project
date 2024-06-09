use crate::mod_char_plugin::helpers::collect_bones;
use crate::mod_char_plugin::helpers::find_child_with_name_containing;
use crate::mod_char_plugin::spawn_scenes::{SceneEntitiesByName, SceneName};
use crate::mod_char_plugin::StateSpawnScene;
use bevy::prelude::*;
use bevy::utils::HashMap;

// Main function will call everyone
pub fn create_mod_player(
    mut commands: Commands,
    all_entities_with_children: Query<&Children>,
    scene_query: Query<(Entity, &SceneName), With<SceneName>>,
    scene_entities_by_name: Res<SceneEntitiesByName>,
    names: Query<&Name>,
    mut next_state: ResMut<NextState<StateSpawnScene>>,
) {
    // Creates modular player
    let main_skeleton_bones = get_main_skeleton_bones_and_armature(
        scene_entities_by_name,
        &all_entities_with_children,
        &names,
    );
    // Attaching bones to our skeleton entity with their morphed meshes
    for (part_scene_entity, part_scene_name) in &scene_query {
        // If statement to attach specific weapons
        if part_scene_name.0 == "sword.glb" {
            let mut sword_entity_commands = commands.entity(part_scene_entity);
            if let Some(handle_bone) = main_skeleton_bones.get("EquipmentHandle.R") {
                sword_entity_commands.set_parent(*handle_bone);
            }
        } else if part_scene_name.0 != "skeleton_female.glb" {
            attach_part_to_main_skeleton(
                &mut commands,
                &all_entities_with_children,
                &names,
                &part_scene_name.0,
                &part_scene_entity,
                &main_skeleton_bones,
            );
        }
    }
    next_state.set(StateSpawnScene::Done);
}

// Will grab the main skeleton entity
pub fn get_main_skeleton_bones_and_armature(
    scene_entities_by_name: Res<SceneEntitiesByName>,
    all_entities_with_children: &Query<&Children>,
    names: &Query<&Name>,
) -> HashMap<String, Entity> {
    let mut main_bones = HashMap::new();

    let main_skeleton_entity = scene_entities_by_name
        .0
        .get("skeleton_female.glb")
        .expect("to have spawned the main skeleton scene");

    let root_bone = find_child_with_name_containing(
        all_entities_with_children,
        &names,
        &main_skeleton_entity,
        "Hips",
    )
    .expect("the skeleton to have a bone called 'Root'");

    collect_bones(
        all_entities_with_children,
        &names,
        &root_bone,
        &mut main_bones,
    );

    println!("Bones in main skeleton: {:#?}", main_bones);

    return main_bones;
}

pub fn attach_part_to_main_skeleton(
    commands: &mut Commands,
    all_entities_with_children: &Query<&Children>,
    names: &Query<&Name>,
    part_scene_name: &String,
    part_scene_entity: &Entity,
    main_skeleton_bones: &HashMap<String, Entity>,
) {
    println!("Attaching loaded_asset part: {}", part_scene_name);

    let root_bone_option = find_child_with_name_containing(
        all_entities_with_children,
        names,
        &part_scene_entity,
        "Hips",
    );
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
