use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_light, spawn_floor));
    }
}

fn spawn_light(mut commands: Commands) {
    let light = (
        PointLightBundle {
            point_light: PointLight {
                intensity: 4000.0,
                color: Color::WHITE,
                ..default()
            },

            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
        Name::new("MainLight"),
    );
    commands.spawn(light);
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // O chao de tudo
    let floor = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(20.0))),
            material: materials.add(Color::DARK_GREEN.into()),
            ..default()
        },
        Name::new("Floor"),
    );
    commands
        .spawn(Collider::cuboid(15.0, 0.0, 15.0))
        .insert(floor);    
    }


// To do scene
// fn spawn_objects(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let mut create_obj =
//         |size: f32, color: Color, name: String, t: Vec3, r: Quat, s: Vec3| -> (PbrBundle, Name) {
//             (
//                 PbrBundle {
//                     mesh: meshes.add(Mesh::from(shape::Cube::new(size))),
//                     material: materials.add(color.into()),
//                     transform: Transform {
//                         translation: t,
//                         rotation: r,
//                         scale: s,
//                     },
//                     ..default()
//                 },
//                 Name::new(name),
//             )
//         };
// }
