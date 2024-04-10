use std::time::Duration;
use bevy::{ prelude::*, time::{Timer,Stopwatch}};
use bevy_rapier3d::prelude::*;
use crate::{camera::CamInfo, world::Ground};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementAction>();
        app.add_systems(Startup, (spawn_hitbox,spawn_others).chain());
        app.add_systems(Update,(display_events,update_grounded,
            keyboard_walk,keyboard_dash,keyboard_jump,
            move_character,apply_movement_damping).chain());
    }
}



#[derive(Event)]
pub enum MovementAction {
    Move(Vec2),
    Dash(Vec2),
    Jump,
}
// Marker component
#[derive(Component)]
pub struct Player;
// Market component
#[derive(Component)]
pub struct PlayerHitbox;
// Check if is on ground
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Component)]
struct Timers{
    up: Stopwatch,
    down: Stopwatch,
    left: Stopwatch,
    right: Stopwatch
}

#[derive(Component)]
struct StatusEffects{
    dash: Timer
}


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

fn spawn_hitbox(mut commands: Commands,assets: Res<AssetServer>){
    
    let player_render = SceneBundle {
        scene: assets.load("start_character.glb#Scene0"),
        ..Default::default()
    };


    commands.spawn(player_render)
    .insert(Player)
    .insert(RigidBody::Dynamic)
    .insert(AdditionalMassProperties::Mass(1.0))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
    .insert(Velocity::zero())
    .insert(Damping {linear_damping:0.0, angular_damping: 0.0})
    .insert(GravityScale(1.0))
    .insert(LockedAxes::ROTATION_LOCKED)
    .with_children(|children| {
        children.spawn(Collider::cylinder(1.83,0.5))
            // Position the collider relative to the rigid-body.
            .insert(PlayerHitbox)
            .insert(TransformBundle::from(Transform::from_xyz(0.0, 1.83, 0.0)))
            .insert(ActiveEvents::COLLISION_EVENTS);
        }
    );

}

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


pub fn update_grounded(rapier_context: Res<RapierContext>,
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


fn keyboard_dash(time: Res<Time>,keys: Res<ButtonInput<KeyCode>>,
    mut q: Query<&mut Timers>,mut movement_event_writer: EventWriter<MovementAction>,
    q_1: Query<&Transform, With<CamInfo>>){

    let mut p_t = q.get_single_mut().unwrap();
    p_t.up.tick(Duration::from_secs_f32(time.delta_seconds()));
    p_t.down.tick(Duration::from_secs_f32(time.delta_seconds()));
    p_t.left.tick(Duration::from_secs_f32(time.delta_seconds()));
    p_t.right.tick(Duration::from_secs_f32(time.delta_seconds()));
    
    let mut direction = Vec2::ZERO;
    let mut dash = false;
    
    let Ok(cam) = q_1.get_single() else{
        return
    }; 

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
        dash = true;
    }
    if keys.just_released(KeyCode::KeyA){
        p_t.left.reset();
    }
    if p_t.left.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyA){
        direction.x =  cam.left().x;
        direction.y =  cam.left().z;
        dash = true;
    }
    if keys.just_released(KeyCode::KeyD){
        p_t.right.reset();
    }
    if p_t.right.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyD){
        direction.x =  cam.right().x;
        direction.y =  cam.right().z;
        dash = true;
    }

    if dash == true{
        movement_event_writer.send(MovementAction::Dash(direction.normalize_or_zero()));
        println!("Just dashed")
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
    mut q_1: Query<&mut Velocity,With<Player>>,){
    for event in movement_event_reader.read() {
        for mut vel in &mut q_1 {
            match event {
                MovementAction::Move(direction) => {
                    vel.linvel.x += direction.x * 20.0 * time.delta_seconds();
                    vel.linvel.z += direction.y * 20.0 * time.delta_seconds();
                }
                MovementAction::Dash(direction)=> {
                    vel.linvel.x += direction.x *300.0 * time.delta_seconds();
                    vel.linvel.z += direction.y * 300.0 * time.delta_seconds();
                }
                MovementAction::Jump =>{
                    vel.linvel.y = 15.0  
                }
            }
        }
    }
}

pub fn apply_movement_damping(mut query: Query<&mut Damping,With<Player>>) {
    for mut damping_factor in &mut query {
        // Todo create state slowing down
        damping_factor.linear_damping = 0.9;
    }
}