use bevy_ecs::prelude::{MessageWriter, Res};
use ferrumc_net::packets::packet_messages::Movement;
use ferrumc_net::SetPlayerRotationPacketReceiver;

/// Handles incoming SetPlayerRotation packets.
/// Simply converts the packet data to a Movement message for unified processing.
pub fn handle(
    receiver: Res<SetPlayerRotationPacketReceiver>,
    mut movement_writer: MessageWriter<Movement>,
) {
    for (event, eid) in receiver.0.try_iter() {
        let movement = Movement::new(eid)
            .rotation((event.yaw, event.pitch).into())
            .on_ground(event.flags & 0x01 != 0);
        movement_writer.write(movement);
    }
}
