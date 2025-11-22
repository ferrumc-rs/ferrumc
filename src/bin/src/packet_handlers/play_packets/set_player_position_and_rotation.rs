use bevy_ecs::prelude::Query;
use bevy_ecs::prelude::{MessageWriter, Res};
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_net::packets::packet_messages::Movement;
use ferrumc_net::SetPlayerPositionAndRotationPacketReceiver;

pub fn handle(
    receiver: Res<SetPlayerPositionAndRotationPacketReceiver>,
    mut transform_event_writer: MessageWriter<Movement>,
    mut query: Query<(&mut Position, &mut Rotation, &mut OnGround)>,
) {
    for (event, eid) in receiver.0.try_iter() {
        // 2. Update the internal Components
        if let Ok((mut pos, mut rot, mut ground)) = query.get_mut(eid) {
            *pos = Position::new(event.x, event.feet_y, event.z);

            *rot = Rotation::new(event.yaw, event.pitch);

            *ground = OnGround::from(event.flags & 0x01 != 0); // Check if the on_ground flag is set

            //TODO: ANTICHEAT
        }
        let movement = Movement::new(eid)
            .position((event.x, event.feet_y, event.z).into())
            .rotation((event.yaw, event.pitch).into());
        transform_event_writer.write(movement);
    }
}
