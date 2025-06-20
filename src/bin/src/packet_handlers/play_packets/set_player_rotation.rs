use bevy_ecs::prelude::{EventWriter, Res};
use ferrumc_net::SetPlayerRotationPacketReceiver;
use ferrumc_net::packets::packet_events::TransformEvent;

pub fn handle(
    events: Res<SetPlayerRotationPacketReceiver>,
    mut event_writer: EventWriter<TransformEvent>,
) {
    for (event, eid) in events.0.try_iter() {
        let transform_event = TransformEvent::new(eid).rotation((event.yaw, event.pitch).into());
        event_writer.write(transform_event);
    }
}
