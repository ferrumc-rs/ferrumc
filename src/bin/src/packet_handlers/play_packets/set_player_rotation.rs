use bevy_ecs::prelude::{MessageWriter, Query, Res};
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_net::packets::packet_messages::Movement;
use ferrumc_net::SetPlayerRotationPacketReceiver;

pub fn handle(
    receiver: Res<SetPlayerRotationPacketReceiver>,
    mut event_writer: MessageWriter<Movement>,
    mut query: Query<&mut Rotation>,
) {
    for (event, eid) in receiver.0.try_iter() {
        // Update the player's components
        if let Ok(mut rotation) = query.get_mut(eid) {
            *rotation = Rotation::new(event.yaw, event.pitch);
        }

        let movement = Movement::new(eid).rotation((event.yaw, event.pitch).into());
        event_writer.write(movement);
    }
}
