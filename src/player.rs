use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement)
            .add_systems(Update, player_jump_dash);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed {
    walk: f32,
}

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    // Objeto de renderizacao
    let player = (
        SceneBundle {
            scene: assets.load("beta.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player,
        Speed { walk: 3.0 },
        ThirdPersonCameraTarget,
        Name::new("Player"),
    );
    commands
        .spawn(RigidBody::Dynamic)
        .insert(player)
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(0.2, 0.5, 0.2))
                .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.5, 0.0)))
                .insert(ColliderMassProperties::Density(2.0))
                .insert(Sleeping::disabled())
                .insert(Ccd::enabled())
                .insert(GravityScale(1.0));
        })
        .insert(Velocity {
            linvel: Vec3::new(0.0, 0.0, 0.0),
            angvel: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(LockedAxes::ROTATION_LOCKED);
}

fn player_movement(
    keys: Res<Input<KeyCode>>,
    mut player_q: Query<(&mut Velocity, &mut Transform, &Speed), With<Player>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    for (mut player_velocity, mut player_transform, player_speed) in player_q.iter_mut() {
        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Erro pegando o objeto de camera: {}", e)).unwrap(),
        };
        let mut direction: Vec3 = Vec3::ZERO;

        if keys.pressed(KeyCode::W) {
            direction = cam.forward();
        }
        if keys.pressed(KeyCode::S) {
            direction = cam.back();
        }
        if keys.pressed(KeyCode::A) {
            direction = cam.left();
        }
        if keys.pressed(KeyCode::D) {
            direction = cam.right();
        }
        if keys.pressed(KeyCode::W) & keys.pressed(KeyCode::A) {
            direction = (cam.forward() + cam.left()) / 2.0;
        }
        if keys.pressed(KeyCode::W) & keys.pressed(KeyCode::D) {
            direction = (cam.forward() + cam.right()) / 2.0;
        }
        if keys.pressed(KeyCode::S) & keys.pressed(KeyCode::A) {
            direction = (cam.back() + cam.left()) / 2.0;
        }
        if keys.pressed(KeyCode::S) & keys.pressed(KeyCode::D) {
            direction = (cam.back() + cam.right()) / 2.0;
        }
        direction.y = 0.0;
        player_velocity.linvel.x = direction.x * player_speed.walk;
        player_velocity.linvel.z = direction.z * player_speed.walk;
        player_velocity.linvel.y += direction.y;
        if direction.length_squared() > 0.0 {
            player_transform.look_to(cam.forward(), Vec3::Y)
        }
    }
}

fn player_jump_dash(
    keys: Res<Input<KeyCode>>,
    mut player_velocity: Query<&mut Velocity, With<Player>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    for mut vel in player_velocity.iter_mut() {
        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Erro pegando o objeto de camera: {}", e)).unwrap(),
        };
        let mut direction = Vec3::new(0.0, 0.0, 0.0);

        if keys.pressed(KeyCode::Space) {
            vel.linvel = Vec3::new(0.0, 10.0, 0.0);
        }
        if keys.just_pressed(KeyCode::W) & keys.pressed(KeyCode::W) {
            direction = cam.forward() * 10.0
        }
        vel.linvel = direction
    }
}
