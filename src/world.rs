use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_light, spawn_floor, spawn_objects));
    }
}

fn spawn_light(mut commands: Commands) {
    let light = (
        PointLightBundle {
            point_light: PointLight {
                intensity: 1200.0,
                color: Color::rgba(0.61, 0.61, 0.61, 1.0),
                shadows_enabled: true,
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
        .spawn(Collider::cuboid(5.0, 0.1, 5.0))
        .insert(floor);
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}

fn spawn_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut create_obj =
        |size: f32, color: Color, name: String, t: Vec3, r: Quat, s: Vec3| -> (PbrBundle, Name) {
            (
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube::new(size))),
                    material: materials.add(color.into()),
                    transform: Transform {
                        translation: t,
                        rotation: r,
                        scale: s,
                    },
                    ..default()
                },
                Name::new(name),
            )
        };

    commands.spawn(create_obj(
        3.0,
        Color::RED,
        "Wall1".to_string(),
        Vec3::new(0., 1.0, 9.7),
        Quat::from_xyzw(0., 0., 0., 0.),
        Vec3::new(6.67, 0.66, 0.2),
    ));
    commands.spawn(create_obj(
        3.0,
        Color::RED,
        "Wall2".to_string(),
        Vec3::new(0., 1.0, -9.7),
        Quat::from_xyzw(0., 0., 0., 0.),
        Vec3::new(6.67, 0.66, 0.2),
    ));
    commands.spawn(create_obj(
        3.0,
        Color::RED,
        "Wall3".to_string(),
        Vec3::new(10., 1.0, 0.),
        Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), 1.55),
        Vec3::new(6.67, 0.66, 0.2),
    ));
    commands.spawn(create_obj(
        3.0,
        Color::RED,
        "Wall4".to_string(),
        Vec3::new(-10., 1.0, 0.),
        Quat::from_axis_angle(Vec3::new(0.0, -1.0, 0.0), 1.55),
        Vec3::new(6.67, 0.66, 0.2),
    ));
}
