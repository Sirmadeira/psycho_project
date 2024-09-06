use bevy::prelude::*;
use lightyear::prelude::server::ControlledBy;
use lightyear::server::events::InputEvent;
use lightyear::shared::tick_manager::TickManager;
use shared::movement_systems::shared_movement_behaviour;
use shared::protocol::Inputs;
use shared::protocol::PlayerPosition;

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
                    shared_movement_behaviour(position, input);
                }
            }
        }
    }
}
