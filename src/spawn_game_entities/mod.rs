use bevy::prelude::*;

use crate::MyAppState;


pub mod lib;
pub mod spawn_world;
pub mod spawn_mod_char;
pub mod spawn_player;
pub mod helpers;


use self::{spawn_world::*,spawn_mod_char::*,spawn_player::*,lib::*};

pub struct SpawnGameEntities;

impl Plugin for SpawnGameEntities {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(MyAppState::InGame),
            (spawn_light, spawn_floor, spawn_wall),
        );
        // Debuging
        // Spawn mod char debug
        app.register_type::<Attachments>();
        app.register_type::<ConfigModularCharacters>();
        app.insert_state(StateSpawnScene::Spawning);
        // Spawn player debug
        app.register_type::<PdInfo>();
        app.register_type::<Timers>();
        app.register_type::<Limit>();
        app.register_type::<Health>();
        // Config resources
        app.insert_resource(AmountPlayers { quantity: 2 });
        app.insert_resource(ConfigModularCharacters {
            visuals_to_be_attached: vec![String::from("rigge_female")],
            weapons_to_be_attached: vec![String::from("katana")],
        });
        // Make skeleton and creates usefull component for animations
        app.add_systems(
            OnEnter(MyAppState::InGame),
            spawn_skeleton_and_attachments
                .chain()
        );
        // Transfer old bones animations to new bones and spit out character to be player
        app.add_systems(
            OnEnter(StateSpawnScene::Spawned),
            (
                transfer_animation,
                make_end_entity,
                disable_culling_for_skinned_meshes,
            ).run_if(all_chars_created)
            .chain()
        );
        //Formulating entity for player movement
        app.add_systems(OnEnter(StateSpawnScene::Done), spawn_main_rigidbody);
    }
}


pub fn all_chars_created(
    skeleton_query: Query<Entity, With<Skeleton>>,
    amount_players: Res<AmountPlayers>,
) -> bool {
    let mut count = 1;
    for _ in skeleton_query.iter() {
        count += 1;
        if count >= amount_players.quantity {
            return true;
        }
    }
    return false;
}

pub fn player_exists(player_q: Query<Entity, With<Player>>) -> bool {
    match player_q.get_single() {
        Ok(_) => true,
        Err(_) => false,
    }
}