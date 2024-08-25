use bevy::prelude::*;
use bevy::utils::Duration;

use crate::form_ingame_camera::setup_entities::CamInfo;
use crate::form_player::setup_entities::*;
use crate::player_mechanics::*;
use crate::treat_animations::lib::AnimationType;

pub fn keyboard_walk(
    keys: Res<ButtonInput<KeyCode>>, // Res<Input<KeyCode>> is used instead of ButtonInput<KeyCode>
    mut movement_event_writer: EventWriter<MovementAction>,
    mut animation_type_writer: EventWriter<AnimationType>,
    q_1: Query<&Transform, With<CamInfo>>,
    q_2: Query<
        (
            Has<StatusEffectDash>,
            Has<StatusEffectStun>,
            Has<StatusEffectAttack>,
            Has<Grounded>,
        ),
        With<Player>,
    >,
) {
    let cam = q_1.get_single().expect("Expected to have a camera");

    let (has_dash, has_stun, has_attack, has_grounded) = q_2
        .get_single()
        .expect("Expected to be able to check if player has dashed");

    if has_dash || has_stun || has_attack {
        return;
    }

    let mut direction = Vec2::ZERO;
    let mut movetype = AnimationType::default();
    let mut key_to_direction =
        |key: KeyCode, cam_dir: Vec3, walk_anim: AnimationType, air_anim: AnimationType| {
            if keys.pressed(key) {
                direction.x = cam_dir.x;
                direction.y = cam_dir.z;
                movetype = if has_grounded { walk_anim } else { air_anim };
            }
        };

    key_to_direction(
        KeyCode::KeyW,
        cam.forward().into(),
        AnimationType::FrontWalk,
        AnimationType::FrontAir,
    );
    key_to_direction(
        KeyCode::KeyS,
        cam.back().into(),
        AnimationType::BackWalk,
        AnimationType::BackAir,
    );
    key_to_direction(
        KeyCode::KeyA,
        cam.left().into(),
        AnimationType::LeftWalk,
        AnimationType::LeftAir,
    );
    key_to_direction(
        KeyCode::KeyD,
        cam.right().into(),
        AnimationType::RightWalk,
        AnimationType::RightAir,
    );

    if direction != Vec2::ZERO {
        movement_event_writer.send(MovementAction::Move(direction.normalize_or_zero()));
        animation_type_writer.send(movetype);
    }
}

pub fn keyboard_dash(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut movement_event_writer: EventWriter<MovementAction>,
    mut animation_type_writer: EventWriter<AnimationType>,
    mut q: Query<(&mut DashTimers, Entity, Has<StatusEffectDash>), With<Player>>,
    q_1: Query<&Transform, With<CamInfo>>,
) {
    for (mut timers, player, has_dashed) in q.iter_mut() {
        let cam = q_1.get_single().expect("To have camera");

        let mut direction = Vec2::ZERO;
        let mut movetype = AnimationType::default();

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
            movetype = AnimationType::FrontDash
        }
        if keys.just_released(KeyCode::KeyS) {
            timers.down.reset();
        }
        if timers.down.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyS) {
            direction.x = cam.back().x;
            direction.y = cam.back().z;
            movetype = AnimationType::BackDash
        }
        if keys.just_released(KeyCode::KeyA) {
            timers.left.reset();
        }
        if timers.left.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyA) {
            direction.x = cam.left().x;
            direction.y = cam.left().z;
            movetype = AnimationType::LeftDash
        }
        if keys.just_released(KeyCode::KeyD) {
            timers.right.reset();
        }
        if timers.right.elapsed_secs() <= 1.0 && keys.just_pressed(KeyCode::KeyD) {
            direction.x = cam.right().x;
            direction.y = cam.right().z;
            movetype = AnimationType::RightDash
        }

        if direction != Vec2::ZERO && has_dashed == false {
            // Add dash status effect
            let status_dash = StatusEffectDash::default();
            commands.entity(player).insert(status_dash);

            // Sending events
            movement_event_writer.send(MovementAction::Dash(direction.normalize_or_zero()));
            animation_type_writer.send(movetype);
        }
    }
}

pub fn keyboard_jump(
    keys: Res<ButtonInput<KeyCode>>,
    mut q_1: Query<(Has<Grounded>, &mut Limit), With<Player>>,
    mut movement_event_writer: EventWriter<MovementAction>,
    mut animation_type_writer: EventWriter<AnimationType>,
) {
    let (is_grounded, mut amount_jumps) = q_1
        .get_single_mut()
        .expect("Player to only have one grounded flag and limit");


    // Forgot to handle just jumped
    if is_grounded && keys.just_pressed(KeyCode::Space) {
        println!("{}",amount_jumps.jump_limit);
        amount_jumps.jump_limit -= 1;
        movement_event_writer.send(MovementAction::Jump);
        animation_type_writer.send(AnimationType::Jump);
    }
    else if keys.just_pressed(KeyCode::Space) && amount_jumps.jump_limit != 0 {
        println!("{}",amount_jumps.jump_limit);
        amount_jumps.jump_limit -= 1;
        movement_event_writer.send(MovementAction::Jump);
        animation_type_writer.send(AnimationType::Jump);
    }
    
    else if is_grounded && amount_jumps.jump_limit == 0 {
        amount_jumps.jump_limit = Limit::default().jump_limit;
    }
}



pub fn keyboard_attack(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut state_attack: Query<(Entity,&mut StateOfAttack), With<Player>>,
    mut animation_type_writer: EventWriter<AnimationType>,
    mut commands: Commands
) {
    let (entity,mut state_attack) = state_attack.get_single_mut().expect("player to only have a single state of attack");



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

    if keys.pressed(KeyCode::KeyW) && mouse.just_pressed(MouseButton::Left){
        let state_of_attack = state_attack.get_attack().expect("Valid string").to_string();

        let name = format!("FrontWalk_{}",state_of_attack);

        animation_type_writer.send(AnimationType::BlendAnimation(name));

        commands.entity(entity).insert(StatusEffectAttack::default());
        
    }

}