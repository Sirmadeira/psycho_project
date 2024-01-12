use bevy::{prelude::*, time::Stopwatch};
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player, spawn_gun))
            .add_systems(Update, (player_movement, player_jump_dash));
    }
}

// Easy way to find player, I am lazy dont want to use ids
#[derive(Component)]
pub struct Player;

// walk = Walking speed, dash = dash speed,
#[derive(Component)]
struct Speeds {
    walk: f32,
    dash: f32,
}

// Check_dash_ = Multiple timer that measure the time a player has pressed a certain keys
// Cd_dash = Cooldown timer that occurs to ensure animation happens and avoid player from dashing mid
// term
#[derive(Component)]
struct Timers {
    check_dash_forward: Stopwatch,
    check_dash_backward: Stopwatch,
    check_dash_left: Stopwatch,
    check_dash_right: Stopwatch,
    cd_dash: Stopwatch,
}

// cd_dash_limit = Time to wait until you can walk after you dash
#[derive(Component)]
struct Limits {
    cd_dash_limit: f32,
    jump_limit: bool,
}

// The swors is essential to player movement
#[derive(Component)]
struct Sword;

fn spawn_gun(mut commands: Commands, assets: Res<AssetServer>) {
    let sword = (
        SceneBundle {
            scene: assets.load("sword.glb#Scene0"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.5, 0.0),
                rotation: Quat::from_axis_angle(Vec3::new(-1.0, 0.0, 0.0), 1.55),
                ..default()
            },
            ..default()
        },
        Sword,
        Name::new("Sword"),
    );
    commands.spawn(sword);
}

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    let player = (
        SceneBundle {
            scene: assets.load("beta.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
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

    commands
        .spawn(RigidBody::Dynamic)
        .insert(player)
        .insert(Timers {
            check_dash_forward: Stopwatch::new(),
            check_dash_backward: Stopwatch::new(),
            check_dash_left: Stopwatch::new(),
            check_dash_right: Stopwatch::new(),
            cd_dash: Stopwatch::new(),
        })
        .insert(Limits {
            cd_dash_limit: 0.75,
            jump_limit: false,
        })
        .with_children(|children| {
            children
                // Body - Collider
                .spawn(Collider::cuboid(0.2, 0.5, 0.2))
                .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.5, 0.0)))
                .insert(ColliderMassProperties::Density(2.0))
                .insert(Sleeping::disabled())
                .insert(Ccd::enabled());
            //Sword - Collider
            children
                .spawn(Collider::cuboid(0.1, 0.2, 0.1))
                .insert(Transform {
                    translation: Vec3::new(0.7, 0.5, 0.7),
                    rotation: Quat::from_axis_angle(
                        Vec3::new(1.0, 0.0, 0.0),
                        std::f32::consts::FRAC_PI_2,
                    ),
                    ..default()
                });
        })
        .insert(Velocity {
            linvel: Vec3::new(0.0, 0.0, 0.0),
            angvel: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(LockedAxes::ROTATION_LOCKED);
}

fn player_movement(
    keys: Res<Input<KeyCode>>,
    mut player_q: Query<(&mut Velocity, &mut Transform, &Speeds, &Timers, &Limits), With<Player>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    for (mut player_velocity, mut player_transform, player_speed, player_cd, player_limit) in
        player_q.iter_mut()
    {
        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Erro pegando o objeto de camera: {}", e)).unwrap(),
        };

        let mut direction: Vec3 = Vec3::ZERO;

        if player_cd.cd_dash.elapsed_secs() > player_limit.cd_dash_limit {
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
        }

        if direction != Vec3::ZERO {
            direction.y = 0.0;
            player_velocity.linvel.x = direction.x * player_speed.walk;
            player_velocity.linvel.z = direction.z * player_speed.walk;
        }

        if direction.length_squared() > 0.0 {
            player_transform.look_to(cam.forward(), Vec3::Y)
        }
    }
}

fn player_jump_dash(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut player_q: Query<
        (
            &mut Velocity,
            &mut Timers,
            &mut Transform,
            &Speeds,
            &mut Limits,
        ),
        With<Player>,
    >,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    for (mut vel, mut timers, mut transform, speeds, mut limits) in player_q.iter_mut() {
        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Erro pegando o objeto de camera: {}", e)).unwrap(),
        };
        timers.check_dash_forward.tick(time.delta());
        timers.check_dash_backward.tick(time.delta());
        timers.check_dash_left.tick(time.delta());
        timers.check_dash_right.tick(time.delta());
        timers.cd_dash.tick(time.delta());

        if timers.cd_dash.elapsed_secs() > limits.cd_dash_limit {
            // Dash mechanics
            if keys.just_pressed(KeyCode::W)
                && timers.check_dash_forward.elapsed_secs() <= limits.cd_dash_limit
            {
                // Limiting because if the player looks at the same transform and dashes forward he
                // bassically flies eternally
                vel.linvel.y += 0.0;
                vel.linvel.x = cam.forward().x * speeds.dash;
                vel.linvel.z = cam.forward().z * speeds.dash;
                timers.cd_dash.reset();
            }
            if keys.just_pressed(KeyCode::W) {
                timers.check_dash_forward.reset();
            }
            if keys.just_pressed(KeyCode::S)
                && timers.check_dash_backward.elapsed_secs() <= limits.cd_dash_limit
            {
                vel.linvel.y += 0.0;
                vel.linvel.x = cam.back().x * speeds.dash;
                vel.linvel.z = cam.back().z * speeds.dash;
                timers.cd_dash.reset();
            }
            if keys.just_pressed(KeyCode::S) {
                timers.check_dash_backward.reset();
            }
            if keys.just_pressed(KeyCode::A)
                && timers.check_dash_left.elapsed_secs() <= limits.cd_dash_limit
            {
                vel.linvel = cam.left() * speeds.dash;
                timers.cd_dash.reset();
            }
            if keys.just_pressed(KeyCode::A) {
                timers.check_dash_left.reset();
            }
            if keys.just_pressed(KeyCode::D)
                && timers.check_dash_right.elapsed_secs() <= limits.cd_dash_limit
            {
                vel.linvel = cam.right() * speeds.dash;
                timers.cd_dash.reset();
            }
            if keys.just_pressed(KeyCode::D) {
                timers.check_dash_right.reset();
            }
        }
        // JUmp mechanics
        if transform.translation.y <= 0.2 && limits.jump_limit == true {
            limits.jump_limit = false
        }
        if keys.just_pressed(KeyCode::Space) && limits.jump_limit == false {
            vel.linvel = cam.up() * speeds.dash;
            if transform.translation.y > 0.2 && keys.just_pressed(KeyCode::Space) {
                vel.linvel = cam.up() * speeds.dash;
                limits.jump_limit = true;
            }
        }
        if vel.linvel.length_squared() > 0.0 {
            transform.look_to(cam.forward(), Vec3::Y)
        }
    }
}
