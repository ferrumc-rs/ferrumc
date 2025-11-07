use bevy_ecs::prelude::{MessageWriter, Res};
use ferrumc_net::packets::packet_messages::Movement;
use ferrumc_net::SetPlayerPositionAndRotationPacketReceiver;

pub fn handle(
    receiver: Res<SetPlayerPositionAndRotationPacketReceiver>,
    mut transform_event_writer: MessageWriter<Movement>,
) {
    for (event, eid) in receiver.0.try_iter() {
        let transform_event = Movement::new(eid)
            .position((event.x, event.feet_y, event.z).into())
            .rotation((event.yaw, event.pitch).into());
        transform_event_writer.write(transform_event);
    }
}
