use bevy_ecs::prelude::Query;
use bevy_ecs::prelude::{EventWriter, Res};
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_net::packets::packet_events::TransformEvent;
use ferrumc_net::SetPlayerPositionAndRotationPacketReceiver;

pub fn handle(
    events: Res<SetPlayerPositionAndRotationPacketReceiver>,
    mut transform_event_writer: EventWriter<TransformEvent>,
    mut query: Query<(&mut Position, &mut Rotation, &mut OnGround)>,
) {
    for (event, eid) in events.0.try_iter() {
        // 2. Update the internal Components
        if let Ok((mut pos, mut rot, mut ground)) = query.get_mut(eid) {
            *pos = Position::new(event.x, event.feet_y, event.z);

            *rot = Rotation::new(event.yaw, event.pitch);

            *ground = OnGround::from(event.flags & 0x01 != 0); // Check if the on_ground flag is set

            //TODO: ANTICHEAT
        }
        let transform_event = TransformEvent::new(eid)
            .position((event.x, event.feet_y, event.z).into())
            .rotation((event.yaw, event.pitch).into());
        transform_event_writer.write(transform_event);
    }
}
