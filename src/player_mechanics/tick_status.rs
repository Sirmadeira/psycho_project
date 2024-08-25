use bevy::prelude::*;
use bevy::utils::Duration;

use crate::form_player::setup_entities::*;
use crate::player_mechanics::*;


// Tick and remove. Time based status
pub fn check_status_ticker(
    time: Res<Time>,
    mut commands: Commands,
    mut q_1: Query<
        (
            Entity,
            Option<&mut StatusEffectDash>,
            Option<&mut StatusEffectStun>,
            Option<&mut StatusEffectAttack>,
        ),
        With<Player>,
    >,
) {
    for (ent, opt_dash, status_effect_stun, opt_attack) in q_1.iter_mut() {
        if let Some(mut status_dash) = opt_dash {
            status_dash
                .0
                .tick(Duration::from_secs_f32(time.delta_seconds()));
            if status_dash.0.just_finished() {
                commands.entity(ent).remove::<StatusEffectDash>();
            }
        }

        if let Some(mut cooldown) = status_effect_stun {
            cooldown
                .timer
                .tick(Duration::from_secs_f32(time.delta_seconds()));
            if cooldown.timer.just_finished() {
                println!("NO longer stunned");
                commands.entity(ent).remove::<StatusEffectStun>();
            }
        }

        if let Some(mut status_attack) = opt_attack {
            status_attack
                .0
                .tick(Duration::from_secs_f32(time.delta_seconds()));
            if status_attack.0.just_finished() {
                println!("No longer attacking");
                commands.entity(ent).remove::<StatusEffectAttack>();
            }
        }


    }
}

