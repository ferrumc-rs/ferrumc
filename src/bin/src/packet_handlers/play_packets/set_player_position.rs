use bevy_ecs::prelude::{MessageWriter, Query, Res};

use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_messages::chunk_calc::ChunkCalc;
use ferrumc_net::packets::packet_messages::Movement;
use ferrumc_net::SetPlayerPositionPacketReceiver;
use tracing::trace;

pub fn handle(
    receiver: Res<SetPlayerPositionPacketReceiver>,
    mut query: Query<(&mut Position, &mut OnGround)>,
    mut movement_messages: MessageWriter<Movement>,
    mut chunk_calc_messages: MessageWriter<ChunkCalc>,
) {
    for (event, eid) in receiver.0.try_iter() {
        if let Ok((mut pos, mut ground)) = query.get_mut(eid) {
            let new_pos = Position::new(event.x, event.feet_y, event.z);

            // Check if chunk changed
            let old_chunk = (pos.x as i32 >> 4, pos.z as i32 >> 4);
            let new_chunk = (new_pos.x as i32 >> 4, new_pos.z as i32 >> 4);
            if old_chunk != new_chunk {
                chunk_calc_messages.write(ChunkCalc(eid));
            }

            // Build movement message with delta BEFORE updating component
            let movement = Movement::new(eid)
                .position_delta_from(&pos, &new_pos)
                .on_ground(event.on_ground);

            // Update components
            *pos = new_pos;
            *ground = OnGround(event.on_ground);

            // Send movement message for broadcasting
            movement_messages.write(movement);

            trace!(
                "Updated position for player {}: ({}, {}, {})",
                eid,
                event.x,
                event.feet_y,
                event.z
            );
        }
    }
}
