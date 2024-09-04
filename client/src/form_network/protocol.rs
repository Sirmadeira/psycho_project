// Also known as contract - It will basically tell me what will be sent from server to client.
// A protocol is composed of

//     Input: Defines the client's input type, i.e. the different actions that a user can perform (e.g. move, jump, shoot, etc.).
//     Message: Defines the message protocol, i.e. the messages that can be exchanged between the client and server.
//     Components: Defines the component protocol, i.e. the list of components that can be replicated between the client and server.
//     Channels: Defines channels that are used to send messages between the client and server.

// Lets us define all player related components

use bevy::ecs::entity::MapEntities;
use bevy::prelude::{
    default, App, Bundle, Component, Deref, DerefMut, Entity, EntityMapper, Plugin, Vec2,
};
use lightyear::client::components::ComponentSyncMode;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul};

// A public crate - Meaning everyone in the client package can easily acess it a new type of infra I didnt know it existed
#[derive(Bundle)]
pub(crate) struct PlayerBundle {
    id: PlayerId,
    position: PlayerPosition,
}

// Id of entitity
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerId(ClientId);

// Current position of player
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Deref, DerefMut)]
pub struct PlayerPosition(Vec2);

impl Add for PlayerPosition {
    type Output = PlayerPosition;
    #[inline]
    fn add(self, rhs: PlayerPosition) -> PlayerPosition {
        PlayerPosition(self.0.add(rhs.0))
    }
}

impl Mul<f32> for &PlayerPosition {
    type Output = PlayerPosition;

    fn mul(self, rhs: f32) -> Self::Output {
        PlayerPosition(self.0 * rhs)
    }
}

// Entities that point to other entities should always have that mapentities impl, because if they dont well you never gonna find that entity in the server
// Because of the way replication works
#[derive(Component, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PlayerParent(Entity);

impl MapEntities for PlayerParent {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.0 = entity_mapper.map_entity(self.0);
    }
}

// Defines channels that are used to send messages between the client and server.
#[derive(Channel)]
pub struct Channel1;

// Defines the message protocol, i.e. the messages that can be exchanged between the client and server.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Message1(pub usize);

//Inputs - Define the actions that the player is currently doing
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Direction {
    pub(crate) up: bool,
    pub(crate) down: bool,
    pub(crate) left: bool,
    pub(crate) right: bool,
}

impl Direction {
    pub(crate) fn is_none(&self) -> bool {
        !self.up && !self.down && !self.left && !self.right
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Inputs {
    Direction(Direction),
    Delete,
    Spawn,
    // None so server can distinguishe from lost packets and none inputs
    None,
}

// Plugin struct that will tel me the whole "contract"
pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        // messages
        app.register_message::<Message1>(ChannelDirection::Bidirectional);
        // inputs
        app.add_plugins(InputPlugin::<Inputs>::default());
        // Tell me exactly what components will be replicated from server to client- Since this is server authoritive
        app.register_component::<PlayerId>(ChannelDirection::ServerToClient)
            .add_prediction(client::ComponentSyncMode::Once)
            .add_interpolation(client::ComponentSyncMode::Once);
        app.register_component::<PlayerPosition>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation(ComponentSyncMode::Full)
            .add_linear_interpolation_fn();
        // channels
        app.add_channel::<Channel1>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });
    }
}
