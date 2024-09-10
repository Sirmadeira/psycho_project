use bevy::prelude::*;
use lightyear::prelude::server::*;
use lightyear::prelude::*;

use crate::shared::protocol::player_structs::*;

use crate::shared;




/// Read client inputs and move players
pub(crate) fn movement(
    mut position_query: Query<(&ControlledBy, &mut PlayerPosition)>,
    mut input_reader: EventReader<InputEvent<Inputs>>,
    tick_manager: Res<TickManager>,
) {
    for input in input_reader.read() {
        let client_id = input.context();
        if let Some(input) = input.input() {
            trace!(
                "Receiving input: {:?} from client: {:?} on tick: {:?}",
                input,
                client_id,
                tick_manager.tick()
            );
            // NOTE: you can define a mapping from client_id to entity_id to avoid iterating through all
            //  entities here
            for (controlled_by, position) in position_query.iter_mut() {
                if controlled_by.targets(client_id) {
                    shared::shared_movement_behaviour(position, input);
                }
            }
        }
    }
}

/// Send messages from server to clients (only in non-headless mode, because otherwise we run with minimal plugins
/// and cannot do input handling)
pub(crate) fn send_message(
    mut server: ResMut<ConnectionManager>,
    input: Option<Res<ButtonInput<KeyCode>>>,
) {
    if input.is_some_and(|input| input.pressed(KeyCode::KeyM)) {
        let message = Message1(5);
        info!("Send message: {:?}", message);
        server
            .send_message_to_target::<Channel1, Message1>(&mut Message1(5), NetworkTarget::All)
            .unwrap_or_else(|e| {
                error!("Failed to send message: {:?}", e);
            });
    }
}
