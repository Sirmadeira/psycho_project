use bevy::prelude::*;

use crate::mod_char_plugin::lib::{AnimationEntityLink, Animations};
use crate::player_effects_plugin::lib::StatePlayerCreation;

pub struct TreatAnimationsPlugin;

impl Plugin for TreatAnimationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationType>();
        app.add_systems(
            Update,
            event_based_animations.run_if(in_state(StatePlayerCreation::Done)),
        );
    }
}

// Tells me which type of movement i should pass, to avoid multiple arguments or enums
#[derive(Event, Debug)]
pub enum AnimationType {
    // If it is forward backwards and so on
    None,
    WalkForward,
    WalkBackward,
    WalkLeft,
    WalkRight,
    LeftAttack,
    RightAttack,
    BackwardAttack,
    ForwardAttack,
    Defend,
    Jump,
    DashForward,
    DashBackward,
    DashLeft,
    DashRight,
    Dead,
}

// Turn this into a state machine later
fn event_based_animations(
    animations: Res<Animations>,
    mut animation_type_event_reader: EventReader<AnimationType>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animation_entity_link: Query<&AnimationEntityLink>,
) {
    for ent in animation_entity_link.iter() {
        // Ent.0 carries the pointer to an  entity with an animation player with this logic we can filter out the entities we want the animation to run
        // Right now runs for everyone
        let mut animation_player = animation_players
            .get_mut(ent.0)
            .expect("That run link animations worked");

        for event in animation_type_event_reader.read() {
            match event {
                AnimationType::WalkForward => {
                    animation_player
                        .play(
                            animations
                                .0
                                .get("TPose")
                                .expect("Walk forward to exist")
                                .clone_weak(),
                        )
                        .set_speed(1.0);
                }
                AnimationType::WalkBackward => {
                    animation_player
                        .play(
                            animations
                                .0
                                .get("TPose")
                                .expect("Run back animation to exist")
                                .clone_weak(),
                        )
                        .set_speed(1.0);
                }
                AnimationType::WalkLeft => {
                    animation_player
                        .play(
                            animations
                                .0
                                .get("TPose")
                                .expect("Run left animation to exist")
                                .clone_weak(),
                        )
                        .set_speed(1.0);
                }
                AnimationType::WalkRight => {
                    animation_player
                        .play(
                            animations
                                .0
                                .get("TPose")
                                .expect("Run right animation to exist")
                                .clone_weak(),
                        )
                        .set_speed(1.0);
                }
                AnimationType::DashForward => {
                    animation_player
                        .play(
                            animations
                                .0
                                .get("TPose")
                                .expect("Run right animation to exist")
                                .clone_weak(),
                        )
                        .set_speed(1.0);
                }
                AnimationType::DashBackward => {
                    animation_player
                        .play(
                            animations
                                .0
                                .get("TPose")
                                .expect("Run right animation to exist")
                                .clone_weak(),
                        )
                        .set_speed(1.0);
                }
                AnimationType::DashLeft => {
                    animation_player
                        .play(
                            animations
                                .0
                                .get("TPose")
                                .expect("Run right animation to exist")
                                .clone_weak(),
                        )
                        .set_speed(1.0);
                }
                AnimationType::DashRight => {
                    animation_player
                        .play(
                            animations
                                .0
                                .get("TPose")
                                .expect("Run right animation to exist")
                                .clone_weak(),
                        )
                        .set_speed(1.0);
                }
                AnimationType::Jump => {
                    animation_player
                        .play(
                            animations
                                .0
                                .get("TPose")
                                .expect("Run right animation to exist")
                                .clone_weak(),
                        )
                        .set_speed(1.0);
                }
                AnimationType::LeftAttack => {
                    animation_player
                        .play(
                            animations
                                .0
                                .get("LeftAttack")
                                .expect("Run right animation to exist")
                                .clone_weak(),
                        )
                        .set_speed(1.0);
                }
                AnimationType::RightAttack => {
                    animation_player
                        .play(
                            animations
                                .0
                                .get("RightAttack")
                                .expect("Run right animation to exist")
                                .clone_weak(),
                        )
                        .set_speed(1.0);
                }
                AnimationType::ForwardAttack => {
                    animation_player
                        .play(
                            animations
                                .0
                                .get("ForwardAttack")
                                .expect("Run right animation to exist")
                                .clone_weak(),
                        )
                        .set_speed(1.0);
                }
                AnimationType::BackwardAttack => {
                    animation_player
                        .play(
                            animations
                                .0
                                .get("BackwardAttack")
                                .expect("Run right animation to exist")
                                .clone_weak(),
                        )
                        .set_speed(1.0);
                }
                AnimationType::Defend => {
                    animation_player
                        .play(
                            animations
                                .0
                                .get("TPose")
                                .expect("Run right animation to exist")
                                .clone_weak(),
                        )
                        .set_speed(1.0);
                }
                AnimationType::Dead => {
                    animation_player
                        .play(
                            animations
                                .0
                                .get("TPose")
                                .expect("Run right animation to exist")
                                .clone_weak(),
                        )
                        .set_speed(1.0);
                }
                _ => {
                    animation_player
                        .play(
                            animations
                                .0
                                .get("TPose")
                                .expect("Idle sword animation to exist")
                                .clone_weak(),
                        )
                        .set_speed(1.0);
                }
            }
        }
    }
}
