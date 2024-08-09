use crate::load_assets_plugin::MyAssets;
use crate::spawn_game_entities::lib::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// Spawns the main collider floor and a ugly mesh
pub fn spawn_floor(
    mut commands: Commands,
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    let collection_gltf = &asset_pack.gltf_files;

    let wood_floor = collection_gltf
        .get("wood_floor.glb")
        .expect("To have floor");

    let test = assets_gltf.get(wood_floor).expect("TO find asset");

    let scene_test = &test.named_scenes["Scene"];

    // for (name,material) in &test.named_materials{
    //     println!("Here is the material {}",name);
    // }
    // BAKE MESH ADD MATERIAL

    let floor = (
        SceneBundle {
            scene: scene_test.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        Name::new("Floor"),
    );
    // He is group 10 because for now we can only have 10 players
    let collider = (
        Collider::cuboid(100.0, 0.5, 100.0),
        Ground,
        CollisionGroups::new(Group::GROUP_10, Group::ALL),
    );

    commands.spawn(floor);
    commands.spawn(collider);
}

pub fn spawn_wall(mut commands: Commands) {
    let wall_collider = (
        Collider::cuboid(1.0, 10.0, 10.0),
        CollisionGroups::new(Group::GROUP_10, Group::ALL),
        Wall,
        ActiveEvents::COLLISION_EVENTS,
    );

    commands
        .spawn(RigidBody::Fixed)
        .insert(SpatialBundle {
            transform: Transform::from_xyz(10.0, 10.0, 10.0),
            ..Default::default()
        })
        .insert(Name::new("Wall"))
        .with_children(|children| {
            children.spawn(wall_collider);
        });
}
