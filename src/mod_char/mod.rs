use crate::{
    mod_char::lib::{AmountPlayers, Attachments, ConfigModularCharacters},
    MyModCharSet,
};

use bevy::prelude::*;

// Making thme public just in case i need to query a specific component or resource for future logic

pub mod helpers;
pub mod lib;
pub mod spawn_modular;

use self::{lib::*, spawn_modular::*};

use crate::MyAppState;

// This plugin creates the character
pub struct ModChar;

impl Plugin for ModChar {
    fn build(&self, app: &mut App) {
        // Debuging
        app.register_type::<Attachments>();
        app.register_type::<ConfigModularCharacters>();
        app.insert_state(StateSpawnScene::Spawning);
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
                .in_set(MyModCharSet::SpawnEntities),
        );
        // Transfer old bones animations to new bones and spit out character to be player
        app.add_systems(
            OnEnter(StateSpawnScene::Spawned),
            (
                transfer_animation,
                make_end_entity,
                disable_culling_for_skinned_meshes,
            )
                .chain()
                .in_set(MyModCharSet::AttachToSkeleton),
        );
        // Sets
        app.configure_sets(OnEnter(MyAppState::InGame), MyModCharSet::SpawnEntities);
        app.configure_sets(
            OnEnter(StateSpawnScene::Spawned),
            MyModCharSet::AttachToSkeleton.run_if(all_chars_created),
        );
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