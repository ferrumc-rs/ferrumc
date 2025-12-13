use bevy_ecs::prelude::{MessageWriter, Res};
use ferrumc_net::packets::packet_messages::Movement;
use ferrumc_net::SetPlayerPositionPacketReceiver;

/// Handles incoming SetPlayerPosition packets.
/// Simply converts the packet data to a Movement message for unified processing.
pub fn handle(
    receiver: Res<SetPlayerPositionPacketReceiver>,
    mut movement_writer: MessageWriter<Movement>,
) {
    for (event, eid) in receiver.0.try_iter() {
        let movement = Movement::new(eid)
            .position((event.x, event.feet_y, event.z).into())
            .on_ground(event.on_ground);
        movement_writer.write(movement);
    }
}
