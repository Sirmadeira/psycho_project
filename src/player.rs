use std::time::Duration;
use bevy::{ prelude::*, time::{Timer,Stopwatch}};
use bevy_rapier3d::prelude::*;
use crate::{camera::CamInfo, world::Ground};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementAction>();
        app.add_systems(Startup, (spawn_hitbox,spawn_others).chain());
        app.add_systems(Update,(
            // Check status effects on player
            check_status_grounded,check_status_effect,
            // Input handler
            keyboard_walk,keyboard_dash,keyboard_jump,
            move_character,apply_movement_damping).chain());

        app.add_systems(Update, display_events);
    }
}

#[derive(Event)]
pub enum MovementAction {
    // Movement direction
    Move(Vec2),
    // Dash direction
    Dash(Vec2),
    // Jump status
    Jump,
}
// Marker component
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerRender;

// Market component
#[derive(Component)]
pub struct PlayerHitbox;

// Check if is on ground
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;
// Check if has dashed
#[derive(Component)]
#[component(storage = "SparseSet")]
struct StatusEffectDash{
    dash_duration: Timer,
}

// Times the dash for each key
#[derive(Component)]
struct Timers{
    up: Stopwatch,
    down: Stopwatch,
    left: Stopwatch,
    right: Stopwatch
}

// Amount of jumps you can have
#[derive(Component)]
struct Limit{
    jump_limit: u8
}

impl Default for Limit {
    fn default() -> Self {
        Self {
            jump_limit: 2
        }
    }
}

// Spawn the hitbox and the player character
fn spawn_hitbox(mut commands: Commands,assets: Res<AssetServer>){
    
    let player_render = SceneBundle {
        scene: assets.load("start_character.glb#Scene0"),
        ..Default::default()
    };


    commands.spawn(RigidBody::Dynamic)
    .insert(Player)
    .insert(AdditionalMassProperties::Mass(1.0))
    .insert(SpatialBundle{
        ..Default::default()
    })
    .insert(Velocity::zero())
    .insert(Damping {linear_damping:0.0, angular_damping: 0.0})
    .insert(GravityScale(1.0))
    .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes:: ROTATION_LOCKED_Z)
    .insert(ExternalImpulse {
        impulse: Vec3::ZERO,
        torque_impulse: Vec3::ZERO,
    })
    .with_children(|children| {
        children.spawn(Collider::cylinder(1.83,0.5))
            // Position the collider relative to the rigid-body.
            .insert(PlayerHitbox)
            .insert(TransformBundle::from(Transform::from_xyz(0.0, 1.83, 0.0)))
            .insert(ActiveEvents::COLLISION_EVENTS);
        }
    )
    .with_children(|children| {
        children.spawn(player_render)
        .insert(PlayerRender);
        }
    );
}

// Spawn other components
fn spawn_others(mut commands: Commands){

    let timers = (Timers {
        up: Stopwatch::new(),
        down: Stopwatch::new(),
        left: Stopwatch::new(),
        right: Stopwatch::new()
        },
        Name::new("DashTimers"));
    
    let limit = Limit {
        ..Default::default()
    };
    
    commands.spawn(timers);
    commands.spawn(limit);    
}

// Usefull info
fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
    ) {
    for collision_event in collision_events.read() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.read() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}

fn keyboard_walk(keys: Res<ButtonInput<KeyCode>>,
    mut movement_event_writer: EventWriter<MovementAction>,
    q_1: Query<&Transform, With<CamInfo>>){


    let Ok(cam) = q_1.get_single() else{
        return
    }; 

    let mut direction = Vec2::ZERO;

    //forward
    if keys.pressed(KeyCode::KeyW) {
        direction.x =  cam.forward().x;
        direction.y =  cam.forward().z;
    }
    // back
    if keys.pressed(KeyCode::KeyS) {
        direction.x =  cam.back().x;
        direction.y =  cam.back().z;
    }
    // left
    if keys.pressed(KeyCode::KeyA) {
        direction.x =  cam.left().x;
        direction.y =  cam.left().z;
    }
    // right
    if keys.pressed(KeyCode::KeyD) {
        direction.x =  cam.right().x;
        direction.y =  cam.right().z;
    }
    if direction != Vec2::ZERO{
        movement_event_writer.send(MovementAction::Move(direction.normalize_or_zero()));
    }
}


fn keyboard_dash(time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut movement_event_writer: EventWriter<MovementAction>,
    mut q: Query<&mut Timers>,
    q_1: Query<&Transform, With<CamInfo>>,
    q_2:Query<(Entity,&Player)>,
    q_3: Query<Has<StatusEffectDash>,With<Player>>,){

    let mut p_t = q.get_single_mut().unwrap();

    let cam = q_1.get_single().unwrap();

    let has_dashed = q_3.get_single().unwrap();


    let mut direction = Vec2::ZERO;
    
    p_t.up.tick(Duration::from_secs_f32(time.delta_seconds()));
    p_t.down.tick(Duration::from_secs_f32(time.delta_seconds()));
    p_t.left.tick(Duration::from_secs_f32(time.delta_seconds()));
    p_t.right.tick(Duration::from_secs_f32(time.delta_seconds()));

    if keys.just_released(KeyCode::KeyW){
        p_t.up.reset();
    }
    if p_t.up.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyW){
        direction.x =  cam.forward().x;
        direction.y =  cam.forward().z;
    }
    if keys.just_released(KeyCode::KeyS){
        p_t.down.reset();
    }
    if p_t.down.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyS){
        direction.x =  cam.back().x;
        direction.y =  cam.back().z;
    }
    if keys.just_released(KeyCode::KeyA){
        p_t.left.reset();
    }
    if p_t.left.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyA){
        direction.x =  cam.left().x;
        direction.y =  cam.left().z;
    }
    if keys.just_released(KeyCode::KeyD){
        p_t.right.reset();
    }
    if p_t.right.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyD){
        direction.x =  cam.right().x;
        direction.y =  cam.right().z;
    }

    if direction != Vec2::ZERO && has_dashed == false {
        movement_event_writer.send(MovementAction::Dash(direction.normalize_or_zero()));
        // Add dash status effect
        let entity_1 = q_2.get_single().unwrap().0;
        let status_dash = StatusEffectDash{dash_duration: Timer::new(Duration::from_secs_f32(2.0), TimerMode::Once)};
        commands.entity(entity_1).insert(status_dash);
    }
}

fn check_status_effect(time: Res<Time>,
    mut commands: Commands,
    mut q_1: Query<(Entity,Option<&mut StatusEffectDash>),With<Player>>){

    for (ent,status) in q_1.iter_mut(){

        if let Some(mut status) = status{
            status.dash_duration.tick(Duration::from_secs_f32(time.delta_seconds()));
            if status.dash_duration.finished(){
                commands.entity(ent).remove::<StatusEffectDash>();
            }
        }
        else {
            return
        }
        
    }
}

pub fn check_status_grounded(rapier_context: Res<RapierContext>,
    mut commands: Commands,
    q_1:Query<(Entity,&PlayerHitbox)>,
    q_2:Query<(Entity,&Ground)>,) {

    let entity1 = q_1.get_single().unwrap().0;// A first entity with a collider attached.
    let entity2 = q_2.get_single().unwrap().0; // A second entity with a collider attached.
    
    /* Find the contact pair, if it exists, between two colliders. */
    if let Some(contact_pair) = rapier_context.contact_pair(entity1, entity2) {
        // The contact pair exists meaning that the broad-phase identified a potential contact.
        if contact_pair.has_any_active_contacts() {
            // The contact pair has active contacts, meaning that it
            // contains contacts for which contact forces were computed.
            commands.entity(entity1).insert(Grounded);
        }
    }
    else {
        commands.entity(entity1).remove::<Grounded>();
    }
}

fn keyboard_jump(keys: Res<ButtonInput<KeyCode>>,
    mut movement_event_writer: EventWriter<MovementAction>,
    q_1: Query<Has<Grounded>,With<PlayerHitbox>>,
    mut q_2:Query<&mut Limit>,){

    let is_grounded = q_1.get_single().unwrap();

    for mut jumps in q_2.iter_mut(){
        if is_grounded{
            jumps.jump_limit = Limit {
                ..Default::default()
            }.jump_limit
        }
        if keys.just_pressed(KeyCode::Space) && jumps.jump_limit != 0{
            jumps.jump_limit -= 1;
            movement_event_writer.send(MovementAction::Jump);
        }
    } 
}

fn move_character(mut movement_event_reader: EventReader<MovementAction>,
    time: Res<Time>,
    mut q_1: Query<(&mut Velocity,&mut ExternalImpulse),(With<Player>,Without<PlayerRender>)>,){
    for event in movement_event_reader.read() {
        for (mut vel,mut status) in &mut q_1 {
            match event {
                MovementAction::Move(direction) => {
                    vel.linvel.x += direction.x * 20.0 * time.delta_seconds();
                    vel.linvel.z += direction.y * 20.0 * time.delta_seconds();
                }
                MovementAction::Dash(direction)=> {
                    status.impulse.x = direction.x * 100.0;
                    status.impulse.z = direction.y * 100.0;
                }
                MovementAction::Jump =>{
                    vel.linvel.y = 15.0  
                }
            }
        }
    }
}

fn apply_movement_damping(mut query: Query<&mut Damping,With<Player>>) {
    for mut damping_factor in &mut query {
        // Todo create state slowing down
        damping_factor.linear_damping = 0.9;
    }
}


// fn player_look_into(mut cam_q: Query<(&mut Transform), With<CamInfo>>,
//     mut player_q: Query<&mut Velocity,With<Player>>){

//     let mut cam_transform = cam_q.get_single_mut().unwrap();
//     let mut player_angvel = player_q.get_single_mut().unwrap();
    

//     cam_transform.rotate_y(angle)
    
    
// }