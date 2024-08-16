use crate::MyAppState;
use bevy::prelude::*;

pub mod helpers;
pub mod lib;
pub mod spawn_mod_char;

use self::{
    lib::*,spawn_mod_char::*,
};


pub struct SpawnGameEntities;

impl Plugin for SpawnGameEntities {
    fn build(&self, app: &mut App) {
        // Creating modular character
        app.add_systems(
            OnEnter(MyAppState::InGame),
            spawn_skeleton_and_attachments.chain(),
        );

        //Create administrative state
        app.insert_state(StateSpawnScene::Spawning);
        // Transfer old bones animations to new bones and spit out character to be player
        app.add_systems(
            OnEnter(StateSpawnScene::Spawned),
            (
                transfer_animation,
                make_end_entity,
                disable_culling_for_skinned_meshes,
            )
                .run_if(all_chars_created)
                .chain(),
        );
        // Amount of player configuration - Tells me how many to spawn
        app.insert_resource(AmountPlayers { quantity: 2 });
        // Tell me what visual and weapons to attack
        app.insert_resource(ConfigModularCharacters {
            visuals_to_be_attached: vec![String::from("rigge_female")],
            weapons_to_be_attached: vec![String::from("katana")],
        });
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

