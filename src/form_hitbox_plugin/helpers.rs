use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::mod_char_plugin::lib::AmountPlayers;

pub fn create_dynamic_collider_groups(player_amount: &Res<AmountPlayers>,collision_number:u32)->CollisionGroups{

    let membership_group =  Group::from_bits(collision_number).expect("TO have at least a membership");
    println!("{:?}",membership_group);

    let mut filter_group = Group::empty();

    for group in (1..=player_amount.quantity).rev(){

        let to_be_group = Group::from_bits(group).expect("Group");

        if membership_group.ne(&to_be_group){
            filter_group |= to_be_group;
            println!("{:?}",group);
        }
    }

    return CollisionGroups::new(membership_group,filter_group);
}