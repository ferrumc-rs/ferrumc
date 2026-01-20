use bevy_ecs::prelude::Query;
use bevy_ecs::prelude::{MessageWriter, Res};
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_messages::chunk_calc::ChunkCalc;
use ferrumc_net::packets::packet_messages::Movement;
use ferrumc_net::SetPlayerPositionAndRotationPacketReceiver;

pub fn handle(
    receiver: Res<SetPlayerPositionAndRotationPacketReceiver>,
    mut movement_messages: MessageWriter<Movement>,
    mut chunk_calc_messages: MessageWriter<ChunkCalc>,
    mut query: Query<(&mut Position, &mut Rotation, &mut OnGround)>,
) {
    for (event, eid) in receiver.0.try_iter() {
        if let Ok((mut pos, mut rot, mut ground)) = query.get_mut(eid) {
            let new_pos = Position::new(event.x, event.feet_y, event.z);
            let new_rot = Rotation::new(event.yaw, event.pitch);
            let on_ground = event.flags & 0x01 != 0;

            // Check if chunk changed
            let old_chunk = (pos.x as i32 >> 4, pos.z as i32 >> 4);
            let new_chunk = (new_pos.x as i32 >> 4, new_pos.z as i32 >> 4);
            if old_chunk != new_chunk {
                chunk_calc_messages.write(ChunkCalc(eid));
            }

            // Build movement message with delta BEFORE updating component
            let movement = Movement::new(eid)
                .position_delta_from(&pos, &new_pos)
                .rotation(new_rot)
                .on_ground(on_ground);

            // Update components
            *pos = new_pos;
            *rot = new_rot;
            *ground = OnGround(on_ground);

            // Send movement message for broadcasting
            movement_messages.write(movement);
        }
    }
}
