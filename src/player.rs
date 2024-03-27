use std::time::Duration;
use bevy::{ input::keyboard::Key, prelude::*, time::Stopwatch};
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementAction>();
        app.add_systems(Startup, (spawn_hitbox,spawn_time).chain());
        app.add_systems(Update,(keyboard_walk,keyboard_dash,move_character).chain());
    }
}


#[derive(Event)]
pub enum MovementAction {
    Move(Vec2),
    Jump,
}


#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerHitbox;

#[derive(Component)]
struct Timers{
    up: Stopwatch,
    down: Stopwatch,
    left: Stopwatch,
    right: Stopwatch,
}



fn spawn_hitbox(mut commands: Commands,assets: Res<AssetServer>){
    let player_render:Handle<Scene> = assets.load("start_character.glb#Scene0");


    commands.spawn(player_render)
    .insert(RigidBody::Dynamic)
    .insert(Player)
    .insert(PlayerHitbox)
    .insert(Velocity::zero())
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
    .insert(GravityScale(0.5))
    .insert(LockedAxes::TRANSLATION_LOCKED | LockedAxes::ROTATION_LOCKED_X)
    .with_children(|children| {
        children.spawn(Collider::cylinder(0.5,0.5))
            // Position the collider relative to the rigid-body.
            .insert(TransformBundle::from(Transform::from_xyz(0.0, 1.0, 0.0)));
        }
    );

}

fn spawn_time(mut commands: Commands){

    let timers = (Timers {
        up: Stopwatch::new(),
        down: Stopwatch::new(),
        left: Stopwatch::new(),
        right: Stopwatch::new()
        },
        Name::new("DashTimers"));
    
    commands.spawn(timers);
    
}


fn keyboard_walk(keys: Res<ButtonInput<KeyCode>>,mut movement_event_writer: EventWriter<MovementAction>,){

    let mut vel = Vec2::ZERO;
    //forward
    if keys.pressed(KeyCode::KeyW) {
        vel =  Vec2::new(0.0,1.0);
    }
    // back
    if keys.pressed(KeyCode::KeyS) {
        vel =  Vec2::new(0.0,-1.0);
    }
    // left
    if keys.pressed(KeyCode::KeyA) {
        vel =  Vec2::new(1.0,0.0);
    }
    // right
    if keys.pressed(KeyCode::KeyD) {
        vel =  Vec2::new(-1.0,0.0);
    }
    if vel != Vec2::ZERO{
        movement_event_writer.send(MovementAction::Move(vel));
    }
}


fn keyboard_dash(time: Res<Time>,keys: Res<ButtonInput<KeyCode>>,
    mut q: Query<&mut Timers>,mut movement_event_writer: EventWriter<MovementAction>,){

    let mut p_t = q.get_single_mut().unwrap();
    p_t.up.tick(Duration::from_secs_f32(time.delta_seconds()));
    p_t.down.tick(Duration::from_secs_f32(time.delta_seconds()));
    p_t.left.tick(Duration::from_secs_f32(time.delta_seconds()));
    p_t.right.tick(Duration::from_secs_f32(time.delta_seconds()));
    
    let mut vel = Vec2::ZERO;

    if keys.just_released(KeyCode::KeyW){
        p_t.up.reset();
    }
    if p_t.up.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyW){
        vel = Vec2::new(0.0,10.0);
    }
    if keys.just_released(KeyCode::KeyS){
        p_t.down.reset();
    }
    if p_t.down.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyS){
        vel = Vec2::new(0.0,-10.0);
    }
    if keys.just_released(KeyCode::KeyA){
        p_t.left.reset();
    }
    if p_t.left.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyA){
        vel = Vec2::new(10.0,0.0);
    }
    if keys.just_released(KeyCode::KeyD){
        p_t.right.reset();
    }
    if p_t.right.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyD){
        vel = Vec2::new(-10.0,0.0);
    }

    if vel != Vec2::ZERO{
        movement_event_writer.send(MovementAction::Move(vel));
    }

    }

fn move_character(mut movement_event_reader: EventReader<MovementAction>,
    time: Res<Time>,
    mut q_1: Query<&mut Velocity,With<PlayerHitbox>>,){
    for event in movement_event_reader.read() {
        for mut vel in &mut q_1 {
            match event {
                MovementAction::Move(direction) => {
                    vel.linvel.x += direction.x * 30.0 * time.delta_seconds();
                    vel.linvel.z += direction.y * 30.0 * time.delta_seconds();
                }
                MovementAction::Jump =>{
                    todo!()
                }
            }
        }
    }
}