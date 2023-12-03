use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player,
        Speed(2.5),
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
    // Filtro que captura so o player object
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    for (mut player_transform, player_speed) in player_q.iter_mut() {
        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Erro pegando o objeto de camera: {}", e)).unwrap(),
        };
        // Pequeno error handler para ver se a key da match
        let mut direction: Vec3 = Vec3::ZERO;
        // AQUI o vetor e igual a zero pq se nao apertar nada bom nao existe nada a ser apertar
        if keys.pressed(KeyCode::W) {
            direction += cam.forward();
        }
        if keys.pressed(KeyCode::S) {
            direction += cam.back();
        }
        if keys.pressed(KeyCode::A) {
            direction += cam.left();
        }
        if keys.pressed(KeyCode::D) {
            direction += cam.right();
        }
        direction.y = 0.0;
        let movement: Vec3 = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
        player_transform.translation += movement;
    }
}
