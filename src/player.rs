use bevy::prelude::*;
use bevy_third_person_camera::*;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    let player = (
        SceneBundle {
            scene: assets.load("beta.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player,
        Speed(2.5),
        ThirdPersonCameraTarget,
        Name::new("Player"),
    );
    commands.spawn(player);
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

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
        if keys.pressed(KeyCode::Space) {
            direction += cam.up();
        }
        let movement: Vec3 = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
        player_transform.translation += movement;
        if direction.length_squared() > 0.0 {
            player_transform.look_to(direction, Vec3::Y)
        }
    }
}
