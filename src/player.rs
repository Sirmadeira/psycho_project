use bevy::{prelude::*,time::Stopwatch};
use bevy_third_person_camera::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, player_movement);
    }
}

// Easy way to find player
#[derive(Component)]
pub struct Player;

// walk = Walking speed, dash = dash speed,
#[derive(Component)]
struct Speeds {
    walk: f32,
    dash: f32,
}

// Check_dash_ = Multiple timer that measure the time a player has pressed a certain keys
// Cd_dash = Cooldown timer that occurs to ensure animation happens
// #[derive(Component)]
// struct Timers {
//     check_dash_forward: Stopwatch,
//     check_dash_backward: Stopwatch,
//     check_dash_left: Stopwatch,
//     check_dash_right: Stopwatch,
//     cd_dash: Stopwatch,
// }

// cd_dash_limit = Time to wait until you can walk after you dash
#[derive(Component)]
struct Limits {
    cd_dash_limit: f32,
    jump_limit: bool,
}

// #[derive(Resource)]
// struct Animations(Vec<Handle<AnimationClip>>);


// Setup main function
fn setup(mut commands: Commands, assets: Res<AssetServer>) {

    let player = (
        SceneBundle {
            scene: assets.load("start_katana.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player,
        Speeds {
            walk: 4.0,
            dash: 8.0,
        },
        ThirdPersonCameraTarget,
        Name::new("Player"),
    );

    let character = (
        SceneBundle {
            scene: assets.load("start_character.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    );

    // Loading the animation data of the players
    // commands.insert_resource(Animations(vec![assets.load("start_character.glb#Animation0")]));
    commands.spawn(character);

    commands
        .spawn(player)
        .insert(Limits {
            cd_dash_limit: 0.75,
            jump_limit: false,
        });
}

// Continous functions

// fn setup_scene_once_loaded(
//     animations: Res<Animations>,
//     mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
// ) {
//     for mut player in &mut players {
//         player.play(animations.0[0].clone_weak()).repeat();
//     }
// }

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<(&mut Transform, &Speeds, &Limits), With<Player>>,
) {
    for (mut player_transform, player_speed, player_limit) in
        player_q.iter_mut()
    {
        let mut direction = Vec3::ZERO;
            if keys.pressed(KeyCode::KeyW) {
                direction += Vec3::new(1.0,0.0,0.0);
            }
            if keys.pressed(KeyCode::KeyS) {
                direction += Vec3::new(-1.0,0.0,0.0);
            }
            if keys.pressed(KeyCode::KeyA) {
                direction += Vec3::new(0.0,0.0,1.0);
            }
            if keys.pressed(KeyCode::KeyD) {
                direction += Vec3::new(0.0,0.0,-1.0);
            }
            if keys.pressed(KeyCode::KeyW) & keys.pressed(KeyCode::KeyA) {
                direction += (Vec3::new(1.0,0.0,0.0) + Vec3::new(0.0,0.0,1.0)) / 2.0;
            }
            if keys.pressed(KeyCode::KeyW) & keys.pressed(KeyCode::KeyD) {
                direction += (Vec3::new(1.0,0.0,0.0) +  Vec3::new(0.0,0.0,-1.0)) / 2.0;
            }
            if keys.pressed(KeyCode::KeyS) & keys.pressed(KeyCode::KeyA) {
                direction += (Vec3::new(-1.0,0.0,0.0) + Vec3::new(0.0,0.0,1.0)) / 2.0;
            }
            if keys.pressed(KeyCode::KeyS) & keys.pressed(KeyCode::KeyD) {
                direction += (Vec3::new(-1.0,0.0,0.0) + Vec3::new(0.0,0.0,1.0)) / 2.0;
            }
        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * player_speed.walk * time.delta_seconds();
        player_transform.translation += movement;
        if direction.length_squared() > 0.0 {
            player_transform.look_to(direction, Vec3::Y)
        }
    }
}

// fn player_jump_dash(
//     time: Res<Time>,
//     keys: Res<Input<KeyCode>>,
//     mut player_q: Query<
//         (
//             &mut Timers,
//             &mut Transform,
//             &Speeds,
//             &mut Limits,
//         ),
//         With<Player>,
//     >,
//     cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
// ) {
//     for ( mut timers, mut transform, speeds, mut limits) in player_q.iter_mut() {
//         let cam = match cam_q.get_single() {
//             Ok(c) => c,
//             Err(e) => Err(format!("Erro pegando o objeto de camera: {}", e)).unwrap(),
//         };
//         timers.check_dash_forward.tick(time.delta());
//         timers.check_dash_backward.tick(time.delta());
//         timers.check_dash_left.tick(time.delta());
//         timers.check_dash_right.tick(time.delta());
//         timers.cd_dash.tick(time.delta());

//         if timers.cd_dash.elapsed_secs() > limits.cd_dash_limit {
//             // Dash mechanics
//             if keys.just_pressed(KeyCode::W)
//                 && timers.check_dash_forward.elapsed_secs() <= limits.cd_dash_limit
//             {
//                 // Limiting because if the player looks at the same transform and dashes forward he
//                 // bassically flies eternally
//                 vel.linvel.x = cam.forward().x * speeds.dash;
//                 vel.linvel.z = cam.forward().z * speeds.dash;
//                 timers.cd_dash.reset();
//             }
//             if keys.just_pressed(KeyCode::W) {
//                 timers.check_dash_forward.reset();
//             }
//             if keys.just_pressed(KeyCode::S)
//                 && timers.check_dash_backward.elapsed_secs() <= limits.cd_dash_limit
//             {
//                 vel.linvel.y += 0.0;
//                 vel.linvel.x = cam.back().x * speeds.dash;
//                 vel.linvel.z = cam.back().z * speeds.dash;
//                 timers.cd_dash.reset();
//             }
//             if keys.just_pressed(KeyCode::S) {
//                 timers.check_dash_backward.reset();
//             }
//             if keys.just_pressed(KeyCode::A)
//                 && timers.check_dash_left.elapsed_secs() <= limits.cd_dash_limit
//             {
//                 vel.linvel = cam.left() * speeds.dash;
//                 timers.cd_dash.reset();
//             }
//             if keys.just_pressed(KeyCode::A) {
//                 timers.check_dash_left.reset();
//             }
//             if keys.just_pressed(KeyCode::D)
//                 && timers.check_dash_right.elapsed_secs() <= limits.cd_dash_limit
//             {
//                 vel.linvel = cam.right() * speeds.dash;
//                 timers.cd_dash.reset();
//             }
//             if keys.just_pressed(KeyCode::D) {
//                 timers.check_dash_right.reset();
//             }
//         }
//         // JUmp mechanics
//         if transform.translation.y <= 0.2 && limits.jump_limit == true {
//             limits.jump_limit = false
//         }
//         if keys.just_pressed(KeyCode::Space) && limits.jump_limit == false {
//             vel.linvel = cam.up() * speeds.dash;
//             if transform.translation.y > 0.2 && keys.just_pressed(KeyCode::Space) {
//                 vel.linvel = cam.up() * speeds.dash;
//                 limits.jump_limit = true;
//             }
//         }

//         if vel.linvel.length_squared() > 0.0 {
//             transform.look_to(cam.forward(), Vec3::Y)
//         }
//     }
// }
