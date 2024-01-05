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

// Os dois compoenntes abaixo foram criados para possibiltiar filtro
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
            transform: Transform::from_xyz(2.0, 0.0, 0.0),
            ..default()
        },
        Player,
        Speed { walk: 5.0 },
        ThirdPersonCameraTarget,
        Name::new("Player"),
    );

    commands
        .spawn(RigidBody::Dynamic)
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(0.2, 0.5, 0.2))
                .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.5, 0.0)))
                .insert(ColliderMassProperties::Density(2.0))
                .insert(Sleeping::disabled())
                .insert(Ccd::enabled())
                .insert(GravityScale(1.0));
        })
        .insert(player)
        .insert(Velocity {
            linvel: Vec3::new(0.0, 0.0, 0.0),
            angvel: Vec3::new(0.0, 0.0, 0.0),
        });
}

fn player_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<(&mut Transform, &Speed), With<Player>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    for (mut player_transform, player_speed) in player_q.iter_mut() {
        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Erro pegando o objeto de camera: {}", e)).unwrap(),
        };
        let mut direction: Vec3 = Vec3::ZERO;
        if keys.pressed(KeyCode::W) {
            direction += cam.forward();
            direction.y = 0.0;
        }
        if keys.pressed(KeyCode::S) {
            direction += cam.back();
            direction.y = 0.0;
        }
        if keys.pressed(KeyCode::A) {
            direction += cam.left();
            direction.y = 0.0;
        }
        if keys.pressed(KeyCode::D) {
            direction += cam.right();
            direction.y = 0.0;
        }
        let movement: Vec3 =
            direction.normalize_or_zero() * player_speed.walk * time.delta_seconds();
        player_transform.translation += movement;
        if direction.length_squared() > 0.0 {
            player_transform.look_to(direction, Vec3::Y)
        }
    }
}

fn player_jump_dash(
    keys: Res<Input<KeyCode>>,
    mut player_velocity: Query<&mut Velocity, With<Player>>,
) {
    for mut vel in player_velocity.iter_mut() {
        if keys.pressed(KeyCode::Space) {
            vel.linvel = Vec3::new(0.0, 10.0, 0.0)
        }
    }
}
