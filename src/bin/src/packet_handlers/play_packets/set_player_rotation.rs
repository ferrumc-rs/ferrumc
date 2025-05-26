use bevy_ecs::prelude::{EventWriter, Res};
use ferrumc_net::packets::packet_events::TransformEvent;
use ferrumc_net::SetPlayerRotationPacketReceiver;

pub fn handle(
    events: Res<SetPlayerRotationPacketReceiver>,
    mut event_writer: EventWriter<TransformEvent>,
) {
    for (event, eid) in events.0.try_iter() {
        let transform_event = TransformEvent::new(eid)
            .rotation((event.yaw, event.pitch).into())
            .on_ground(event.on_ground);
        event_writer.write(transform_event);
    }
}
