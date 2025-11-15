use bevy_ecs::prelude::{MessageWriter, Res};
use ferrumc_net::packets::packet_messages::Movement;
use ferrumc_net::SetPlayerRotationPacketReceiver;

pub fn handle(
    receiver: Res<SetPlayerRotationPacketReceiver>,
    mut event_writer: MessageWriter<Movement>,
) {
    for (event, eid) in receiver.0.try_iter() {
        let transform_event = Movement::new(eid).rotation((event.yaw, event.pitch).into());
        event_writer.write(transform_event);
    }
}
