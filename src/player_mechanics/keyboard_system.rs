use bevy::a11y::accesskit::Action;
use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_rapier3d::prelude::Velocity;

use crate::form_ingame_camera::setup_entities::CamInfo;
use crate::form_player::setup_entities::*;
use crate::player_mechanics::*;
use crate::treat_animations::lib::AnimationType;
pub fn keyboard_walk(
    keys: Res<ButtonInput<KeyCode>>,  // Adjusted to Res<Input<KeyCode>> instead of ButtonInput<KeyCode>>
    mut movement_event_writer: EventWriter<MovementAction>,
    mut player_action_writer: EventWriter<PlayerAction>,
    cam_query: Query<&Transform, With<CamInfo>>,
    player_status_query: Query<
        (
            Has<StatusEffectDash>,
            Has<StatusEffectStun>,
            Has<StatusEffectAttack>,
            Has<Grounded>,
        ),
        With<Player>,
    >,
) {
    let cam = cam_query.get_single().expect("Expected to have a camera");
    let (has_dash, has_stun, has_attack, has_grounded) = player_status_query
        .get_single()
        .expect("Expected to be able to check player status");

    let mut direction = Vec2::ZERO;
    let mut player_action: Option<PlayerAction> = None;

    let mut key_to_direction = |key: KeyCode, cam_dir: Vec3, walk_action: PlayerAction, air_action: PlayerAction| {
        if keys.pressed(key) {
            direction.x = cam_dir.x;
            direction.y = cam_dir.z;
            player_action = Some(if has_grounded { walk_action } else { air_action });
        }
    };

    key_to_direction(
        KeyCode::KeyW,
        cam.forward().into(),
        PlayerAction::FrontWalk,
        PlayerAction::FrontAir,
    );
    key_to_direction(
        KeyCode::KeyS,
        cam.back().into(),
        PlayerAction::BackWalk,
        PlayerAction::BackAir,
    );
    key_to_direction(
        KeyCode::KeyA,
        cam.left().into(),
        PlayerAction::LeftWalk,
        PlayerAction::LeftAir,
    );
    key_to_direction(
        KeyCode::KeyD,
        cam.right().into(),
        PlayerAction::RightWalk,
        PlayerAction::RightAir,
    );

    if direction != Vec2::ZERO {
        movement_event_writer.send(MovementAction::Move(direction.normalize_or_zero()));
        if !has_attack || has_dash || has_stun {
            if let Some(action) = player_action {
                player_action_writer.send(action);
            }
        }
    }
}


pub fn keyboard_dash(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut movement_event_writer: EventWriter<MovementAction>,
    mut player_action_writer: EventWriter<PlayerAction>,
    mut q: Query<(&mut DashTimers, Entity, Has<StatusEffectDash>), With<Player>>,
    q_1: Query<&Transform, With<CamInfo>>,
) {
    for (mut timers, player, has_dashed) in q.iter_mut() {
        let cam = q_1.get_single().expect("To have camera");

        let mut direction = Vec2::ZERO;
        let mut player_action: Option<PlayerAction> = None;

        timers
            .up
            .tick(Duration::from_secs_f32(time.delta_seconds()));
        timers
            .down
            .tick(Duration::from_secs_f32(time.delta_seconds()));
        timers
            .left
            .tick(Duration::from_secs_f32(time.delta_seconds()));
        timers
            .right
            .tick(Duration::from_secs_f32(time.delta_seconds()));

        if keys.just_released(KeyCode::KeyW) {
            timers.up.reset();
        }
        if timers.up.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyW) {
            direction.x = cam.forward().x;
            direction.y = cam.forward().z;
            player_action = Some(PlayerAction::FrontDash)
        }
        if keys.just_released(KeyCode::KeyS) {
            timers.down.reset();
        }
        if timers.down.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyS) {
            direction.x = cam.back().x;
            direction.y = cam.back().z;
            player_action = Some(PlayerAction::BackDash)
        }
        if keys.just_released(KeyCode::KeyA) {
            timers.left.reset();
        }
        if timers.left.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyA) {
            direction.x = cam.left().x;
            direction.y = cam.left().z;
            player_action = Some(PlayerAction::LeftDash)
        }
        if keys.just_released(KeyCode::KeyD) {
            timers.right.reset();
        }
        if timers.right.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyD) {
            direction.x = cam.right().x;
            direction.y = cam.right().z;
            player_action = Some(PlayerAction::RightDash)
        }

        if direction != Vec2::ZERO && has_dashed == false {
            // Add dash status effect
            let status_dash = StatusEffectDash::default();
            commands.entity(player).insert(status_dash);

            // Sending events
            movement_event_writer.send(MovementAction::Dash(direction.normalize_or_zero()));
            if let Some(action) = player_action{
                player_action_writer.send(action);
            }

        }
    }
}

pub fn keyboard_jump(
    keys: Res<ButtonInput<KeyCode>>,
    mut q_1: Query<(Has<Grounded>, &mut Limit), With<Player>>,
    mut movement_event_writer: EventWriter<MovementAction>,
    mut player_action_writer: EventWriter<PlayerAction>,
) {
    let (is_grounded, mut amount_jumps) = q_1
        .get_single_mut()
        .expect("Player to only have one grounded flag and limit");


    // Forgot to handle just jumped
    if is_grounded && keys.just_pressed(KeyCode::Space) {
        println!("{}",amount_jumps.jump_limit);
        amount_jumps.jump_limit -= 1;
        movement_event_writer.send(MovementAction::Jump);
        player_action_writer.send(PlayerAction::Jump);
    }
    else if keys.just_pressed(KeyCode::Space) && amount_jumps.jump_limit != 0 {
        println!("{}",amount_jumps.jump_limit);
        amount_jumps.jump_limit -= 1;
        movement_event_writer.send(MovementAction::Jump);
        player_action_writer.send(PlayerAction::Jump);
    }
    
    else if is_grounded && amount_jumps.jump_limit == 0 {
        amount_jumps.jump_limit = Limit::default().jump_limit;
    }
}



pub fn keyboard_attack(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    status: Query<(Has<StatusEffectAttack>),With<Player>>,
    mut state_attack: Query<(Entity,&Velocity,&mut StateOfAttack), With<Player>>,
    mut player_action_writer: EventWriter<PlayerAction>,
    mut commands: Commands
) {
    let (entity,vel,mut state_attack) = state_attack.get_single_mut().expect("player to only have a single state of attack");

    let has_attacked = status.get_single().expect("To only have one player");

    // Handling KeyCode::KeyE
    if keys.just_pressed(KeyCode::KeyE) {
        if state_attack.index == 0{
           state_attack.index = 1
        }
        else if state_attack.index == 1{
            state_attack.index = 0
        }
        else if state_attack.index == 2{
            state_attack.index = 0
        }
        else if state_attack.index == 3{
            state_attack.index = 0
        }
    }

    // Handling KeyCode::KeyQ
    if keys.just_pressed(KeyCode::KeyQ) {
        if state_attack.index == 0{
            state_attack.index = 2
        }
        else if state_attack.index == 1{
            state_attack.index = 2
        }
        else if state_attack.index == 2{
            state_attack.index = 3
        }
        else if state_attack.index == 3{
            state_attack.index = 2
         }
    }


    if vel.linvel.length() < 0.03 && mouse.just_pressed(MouseButton::Left){
        let state_of_attack = state_attack.get_attack().expect("Valid string").to_string();        
        let name = format!("Idle_{}",state_of_attack);
        if !has_attacked{
            commands.entity(entity).insert(StatusEffectAttack::default());
            player_action_writer.send(PlayerAction::BlendAnimation(name));
        }
        else {
            println!("Todo combo");
        }
    }

    if keys.pressed(KeyCode::KeyW) && mouse.just_pressed(MouseButton::Left){
        let state_of_attack = state_attack.get_attack().expect("Valid string").to_string();        
        let name = format!("FrontWalk_{}",state_of_attack);
        if !has_attacked{
            commands.entity(entity).insert(StatusEffectAttack::default());
            player_action_writer.send(PlayerAction::BlendAnimation(name));
        }
        else {
            println!("Todo combo");
        }
    }


    if keys.pressed(KeyCode::KeyS) && mouse.just_pressed(MouseButton::Left){
        let state_of_attack = state_attack.get_attack().expect("Valid string").to_string();        
        let name = format!("BackWalk_{}",state_of_attack);
        if !has_attacked{
            commands.entity(entity).insert(StatusEffectAttack::default());
            player_action_writer.send(PlayerAction::BlendAnimation(name));
        }
        else {
            println!("Todo combo");
        }
    }
    if keys.pressed(KeyCode::KeyA) && mouse.just_pressed(MouseButton::Left){
        let state_of_attack = state_attack.get_attack().expect("Valid string").to_string();        
        let name = format!("LeftWalk_{}",state_of_attack);
        if !has_attacked{
            commands.entity(entity).insert(StatusEffectAttack::default());
            player_action_writer.send(PlayerAction::BlendAnimation(name));
        }
        else {
            println!("Todo combo");
        }
    }
    if keys.pressed(KeyCode::KeyD) && mouse.just_pressed(MouseButton::Left){
        let state_of_attack = state_attack.get_attack().expect("Valid string").to_string();        
        let name = format!("RightWalk_{}",state_of_attack);
        if !has_attacked{
            commands.entity(entity).insert(StatusEffectAttack::default());
            player_action_writer.send(PlayerAction::BlendAnimation(name));
        }
        else {
            println!("Todo combo");
        }
    }

}




pub fn player_state(
    player: Query<Entity,With<Player>>,
    player_velocity: Query<&Velocity,With<Player>>,
    
    mut status_idle: Query<&mut StatusIdle,With<Player>>,
    mut status_attack:Query<(&StateOfAttack,&mut StatusEffectAttack),With<Player>>,
    
    mut send_animation: EventWriter<AnimationType>,
    mut read_player_action: EventReader<PlayerAction>,
    time: ResMut<Time>,
    mut commands: Commands
){

    let mut player_commands = commands.entity(player.get_single().expect("Player to have velocity"));

    let velocity = player_velocity.get_single().expect("Player to have velocity");


    // ATTACK STUN DASH MUST OCCUR FIRST


    // Status idle
    if velocity.linvel.length_squared() < 0.1 {
        if let Ok(mut idle) = status_idle.get_single_mut(){
            idle
            .timer
            .tick(Duration::from_secs_f32(time.delta_seconds()));
            if idle.timer.just_finished(){
                println!("player idle sending animation");
                send_animation.send(AnimationType(ActionProperties{
                    name: "Idle".to_string(),
                    duration: Duration::from_secs(2),
                    repeat: false
                }));
            }
        }
        else {
            println!("Inserting idle");
            player_commands.insert(StatusIdle::default());  
            println!("Make in idle state")              
        }
    }

        
    for player_action in read_player_action.read(){

        let animation_properties = player_action.properties();

        if animation_properties.name.contains("Jump") {
            player_commands.remove::<StatusIdle>(); 
            send_animation.send(AnimationType(animation_properties.clone()));
            println!("Uhandled animation {}",animation_properties.name);
        }   

        if velocity.linvel.length_squared() >= 0.1 {
            if animation_properties.name.contains("Walk") || animation_properties.name.contains("Air"){
                send_animation.send(AnimationType(animation_properties.clone()));
                println!("player walking sending animation"); 
                player_commands.remove::<StatusIdle>();     
            }
        }



    }





}