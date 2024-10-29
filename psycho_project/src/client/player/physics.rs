use crate::shared::protocol::player_structs::Direction;
use crate::shared::protocol::player_structs::*;
use crate::shared::shared_behavior::CharacterPhysicsBundle;
use bevy::prelude::*;
use lightyear::client::input::native::*;
use lightyear::client::prediction::Predicted;
use lightyear::shared::tick_manager::TickManager;

pub struct PlayerPhysicsPlugin;

impl Plugin for PlayerPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedPreUpdate,
            buffer_input.in_set(InputSystemSet::BufferInputs),
        );
        // Add physical components to predicted players
        app.add_systems(Update, add_physics_to_players);
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
