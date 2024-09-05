use crate::form_modular_char::lib::AmountPlayers;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn create_dynamic_collider_groups(
    player_amount: &Res<AmountPlayers>,
    collision_number: u32,
    base_group: Option<Group>,
) -> CollisionGroups {
    let membership_group;
    let mut filter_group;

    // Only for weapons, may extend later
    if let Some(base_group) = base_group {
        membership_group =
            Group::from_bits(player_amount.quantity + 1).expect("TO have at least a membership");
        filter_group = base_group;
    } else {
        membership_group =
            Group::from_bits(collision_number).expect("TO have at least a membership");
        filter_group = Group::empty();
    }

    for group in (1..=player_amount.quantity).rev() {
        let to_be_group = Group::from_bits(group).expect("Group");
        if membership_group.ne(&to_be_group) {
            filter_group = filter_group | to_be_group;
        }
    }

    return CollisionGroups::new(membership_group, filter_group);
}
