use bevy::prelude::*;

fn name_morphs(
    mut has_printed: Local<bool>,
    morph_data: Res<MorphData>,
    meshes: Res<Assets<Mesh>>,
) {
    if *has_printed {
        return;
    }

    let Some(mesh) = meshes.get(&morph_data.mesh) else {
        return;
    };
    let Some(names) = mesh.morph_target_names() else {
        return;
    };
    for name in names {
        println!("  {name}");
    }
}