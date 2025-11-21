use bevy_ecs::prelude::{EventWriter, Query, Res};
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_net::packets::packet_events::TransformEvent;
use ferrumc_net::SetPlayerRotationPacketReceiver;

pub fn handle(
    events: Res<SetPlayerRotationPacketReceiver>,
    mut event_writer: EventWriter<TransformEvent>,
    mut query: Query<&mut Rotation>,
) {
    for (event, eid) in events.0.try_iter() {
        // Update the player's components
        if let Ok(mut rotation) = query.get_mut(eid) {
            *rotation = Rotation::new(event.yaw, event.pitch);
        }

        let transform_event = TransformEvent::new(eid).rotation((event.yaw, event.pitch).into());
        event_writer.write(transform_event);
    }
}
