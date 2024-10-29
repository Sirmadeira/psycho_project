use crate::shared::protocol::player_structs::Direction;
use crate::shared::protocol::player_structs::*;
use crate::shared::protocol::world_structs::FloorMarker;
use crate::shared::shared_behavior::{shared_movement_behaviour, CharacterPhysicsBundle};
use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, Velocity};
use lightyear::client::events::InputEvent;
use lightyear::client::input::native::*;
use lightyear::client::prediction::Predicted;
use lightyear::shared::replication::components::Replicated;
use lightyear::shared::tick_manager::TickManager;

pub struct PlayerPhysicsPlugin;

impl Plugin for PlayerPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedPreUpdate,
            buffer_input.in_set(InputSystemSet::BufferInputs),
        );
        app.add_systems(FixedUpdate, player_movement);

        app.add_systems(Update, add_physics_to_players);
        app.add_systems(Update, spawn_world);
    }
}

fn add_physics_to_players(
    players: Query<Entity, (Added<Predicted>, With<PlayerId>)>,
    mut commands: Commands,
) {
    for player in players.iter() {
        commands
            .entity(player)
            .insert(CharacterPhysicsBundle::default());
    }
}

fn buffer_input(
    tick_manager: Res<TickManager>,
    mut input_manager: ResMut<InputManager<Inputs>>,
    keypress: Res<ButtonInput<KeyCode>>,
) {
    let tick = tick_manager.tick();
    let mut input = Inputs::None;
    let mut direction = Direction {
        forward: false,
        down: false,
        left: false,
        right: false,
    };
    if keypress.pressed(KeyCode::KeyW) || keypress.pressed(KeyCode::ArrowUp) {
        direction.forward = true;
    }
    if keypress.pressed(KeyCode::KeyS) || keypress.pressed(KeyCode::ArrowDown) {
        direction.down = true;
    }
    if keypress.pressed(KeyCode::KeyA) || keypress.pressed(KeyCode::ArrowLeft) {
        direction.left = true;
    }
    if keypress.pressed(KeyCode::KeyD) || keypress.pressed(KeyCode::ArrowRight) {
        direction.right = true;
    }
    if !direction.is_none() {
        input = Inputs::Direction(direction);
    }
    input_manager.add_input(input, tick)
}

fn player_movement(
    mut position_query: Query<&mut Velocity, With<Predicted>>,
    mut input_reader: EventReader<InputEvent<Inputs>>,
) {
    for input in input_reader.read() {
        if let Some(input) = input.input() {
            //No need to iterate the position when the input is None
            if input == &Inputs::None {
                continue;
            }
            for position in position_query.iter_mut() {
                shared_movement_behaviour(position, input);
            }
        }
    }
}

fn spawn_world(
    floor: Query<Entity, (Added<Replicated>, With<FloorMarker>)>,
    mut commands: Commands,
) {
    if let Ok(floor) = floor.get_single() {
        info!("Spawning physical floor");
        // Usually it is recommended that this is a shared bundle but for now fuck it
        let collider = Collider::cuboid(100.0, 0.5, 100.0);
        let name = Name::new("PhysicalFloor");
        commands.entity(floor).insert(collider).insert(name);
    }
}
