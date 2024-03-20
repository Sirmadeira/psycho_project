use bevy::{ input::keyboard::Key, prelude::*, time::Stopwatch};
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
// Cd_dash = Cd dash cooldown of the dash
#[derive(Component)]
struct Timers {
    check_dash_forward: Stopwatch,
    check_dash_backward: Stopwatch,
    check_dash_left: Stopwatch,
    check_dash_right: Stopwatch,
    cd_dash: Stopwatch,
}

// cd_dash_limit = Time that the key waits for you to dash
#[derive(Component)]
struct Limits {
    cd_dash_limit: f32,
    jump_limit: bool,
}

#[derive(Component)]
struct DashFlags{
    forward:bool,
    backward:bool,
    left:bool,
    right:bool
}

// #[derive(Resource)]
// struct Animations(Vec<Handle<AnimationClip>>);


// Setup main function
fn setup(mut commands: Commands, assets: Res<AssetServer>) {

    let player = (
        SceneBundle {
            scene: assets.load("start_character.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player,
        Speeds {
            walk: 4.0,
            dash: 20.0,
        },
        Limits {
            cd_dash_limit: 2.0,
            jump_limit: false,
        },
        ThirdPersonCameraTarget,
        Name::new("Player"),
        Timers{
            check_dash_forward:Stopwatch::new(),
            check_dash_backward:Stopwatch::new(),
            check_dash_left:Stopwatch::new(),
            check_dash_right:Stopwatch::new(),
            cd_dash:Stopwatch::new()
        },
        DashFlags{
            forward:false,
            backward:false,
            left:false,
            right:false
        }
    );
    commands
        .spawn(player);
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
    mut player_q: Query<(&mut Transform, &Speeds,&mut Timers,&Limits,& mut DashFlags), With<Player>>,
) {
    for (mut player_transform, player_speed,mut player_timers,player_limits,mut player_dash_flags) in
        player_q.iter_mut()
    {   
        player_timers.check_dash_forward.tick(time.delta());
        player_timers.check_dash_backward.tick(time.delta());
        player_timers.check_dash_left.tick(time.delta());
        player_timers.check_dash_right.tick(time.delta());
        player_timers.cd_dash.tick(time.delta());


        let mut direction = Vec3::ZERO;

        if player_dash_flags.forward == true{
            println!("Player is dashing");
            direction += Vec3::new(1.0,0.0,0.0).normalize_or_zero() * player_speed.dash * time.delta_seconds();
            if player_timers.cd_dash.elapsed_secs() >= player_limits.cd_dash_limit{
                player_dash_flags.forward = false;
                println!("Player can dash again");
            }
        }

        if keys.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0,0.0,1.0).normalize_or_zero() * player_speed.walk * time.delta_seconds();
            println!("Pressed W");
        }
        if keys.just_released(KeyCode::KeyW) {
            player_timers.check_dash_forward.reset();
            println!("Just released w quickly");
        }

        if keys.just_pressed(KeyCode::KeyW) && player_timers.check_dash_forward.elapsed_secs() <= 0.25 && player_dash_flags.forward == false{
            direction += Vec3::new(0.0,0.0,1.0).normalize_or_zero() * player_speed.dash * time.delta_seconds();
            player_dash_flags.forward = true;
            player_timers.cd_dash.reset();
        }

        direction.y = 0.0;
        let movement = direction;
        player_transform.translation += movement;

    }   
}

// fn player_jump_dash(
//     time: Res<Time>,
//     keys: Res<ButtonInput<KeyCode>>,
//     mut player_q: Query<
//         (
//             &mut Timers,
//             &mut Transform,
//             &Speeds,
//             &Limits,
//             &mut DashFlags
//         ),
//         With<Player>,
//     >,
// ) {
//     for ( mut timers, mut player_transform, player_speed, limits,mut dash_flags) in player_q.iter_mut() {

//         // Timers
//         timers.check_dash_forward.tick(time.delta());
//         timers.check_dash_backward.tick(time.delta());
//         timers.check_dash_left.tick(time.delta());
//         timers.check_dash_right.tick(time.delta());
//         timers.cd_dash.tick(time.delta());

//         // Check if wants to dash
//         if keys.just_pressed(KeyCode::KeyW)
//             && timers.check_dash_forward.elapsed_secs() <= limits.cd_dash_limit
//         {
//             timers.cd_dash.reset();
//             dash_flags.forward = true;
//             println!("O seu player deu dash")
//         }
//         if keys.just_pressed(KeyCode::KeyW) && timers.cd_dash.elapsed_secs() >= limits.cd_dash_limit {
//             timers.check_dash_forward.reset();
//             println!("pressionou")
//         }
//         if keys.just_pressed(KeyCode::KeyS)
//             && timers.check_dash_backward.elapsed_secs() <= limits.cd_dash_limit
//         {
//             timers.cd_dash.reset();
//             dash_flags.backward = true;
//         }
//         if keys.just_pressed(KeyCode::KeyS) && timers.cd_dash.elapsed_secs() >= limits.cd_dash_limit {
//             timers.check_dash_backward.reset();
//         }
//         if keys.just_pressed(KeyCode::KeyA)
//             && timers.check_dash_left.elapsed_secs() <= limits.cd_dash_limit
//         {
//             timers.cd_dash.reset();
//             dash_flags.left = true;
//         }
//         if keys.just_pressed(KeyCode::KeyA) && timers.cd_dash.elapsed_secs() >= limits.cd_dash_limit {
//             timers.check_dash_left.reset();
//         }
//         if keys.just_pressed(KeyCode::KeyD)
//             && timers.check_dash_right.elapsed_secs() <= limits.cd_dash_limit
//         {
//             timers.cd_dash.reset();
//             dash_flags.right = true;
//         }
//         if keys.just_pressed(KeyCode::KeyD) && timers.cd_dash.elapsed_secs() >= limits.cd_dash_limit {
//             timers.check_dash_right.reset();
//     }
//     if dash_flags.forward == true{
//         player_transform.translation += Vec3::new(0.0,0.0,1.0).normalize_or_zero() * player_speed.dash * time.delta_seconds();
//         if timers.cd_dash.elapsed_secs() >= limits.cd_dash_limit {
//             dash_flags.forward = false;
//         }
//     }
//     if dash_flags.backward == true{
//         player_transform.translation += Vec3::new(0.0,0.0,-1.0).normalize_or_zero() * player_speed.dash * time.delta_seconds();
//         if timers.cd_dash.elapsed_secs() >= limits.cd_dash_limit {
//             dash_flags.backward = false;
//         }
//     }
//     if dash_flags.left == true{
//         player_transform.translation += Vec3::new(1.0,0.0,0.0).normalize_or_zero() * player_speed.dash * time.delta_seconds();
//         if timers.cd_dash.elapsed_secs() >= limits.cd_dash_limit {
//             dash_flags.left = false;
//         }
//     }

//     if dash_flags.right == true{
//         player_transform.translation += Vec3::new(-1.0,0.0,0.0).normalize_or_zero() * player_speed.dash * time.delta_seconds();
//         if timers.cd_dash.elapsed_secs() >= limits.cd_dash_limit {
//             dash_flags.right = false;
//         }
//     }
// }
// }
