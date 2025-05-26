use bevy_ecs::prelude::{EventWriter, Res};
use ferrumc_net::packets::packet_events::TransformEvent;
use ferrumc_net::SetPlayerPositionAndRotationPacketReceiver;

pub fn handle(
    events: Res<SetPlayerPositionAndRotationPacketReceiver>,
    mut transform_event_writer: EventWriter<TransformEvent>,
) {
    for (event, eid) in events.0.try_iter() {
        let transform_event = TransformEvent::new(eid)
            .position((event.x, event.feet_y, event.z).into())
            .rotation((event.yaw, event.pitch).into())
            .on_ground(event.on_ground);
        transform_event_writer.write(transform_event);
    }
}
