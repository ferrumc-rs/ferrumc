use std::time::Instant;

use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{EventWriter, Res};
use bevy_ecs::system::Query;
use ferrumc_net::packets::packet_events::TransformEvent;
use ferrumc_net::SetPlayerPositionAndRotationPacketReceiver;
use ferrumc_pdc::container::PersistentDataContainer;
use ferrumc_pdc::PersistentKey;

pub fn handle(
    events: Res<SetPlayerPositionAndRotationPacketReceiver>,
    mut pdc_query: Query<(&mut PersistentDataContainer, Entity)>,
    mut transform_event_writer: EventWriter<TransformEvent>,
) {
    for (mut container, _) in pdc_query.iter_mut() {
        let instant = Instant::now();
        let key = PersistentKey::<i32>::new("counter");
        let grabbed = container.get_or(&key, 0);
        let counter = grabbed + 1;
        container.set(&key, counter).unwrap();

        println!(
            "PDC: Updated counter: {} (Elapsed: {:?})",
            counter,
            instant.elapsed()
        );
    }

    for (event, eid) in events.0.try_iter() {
        let transform_event = TransformEvent::new(eid)
            .position((event.x, event.feet_y, event.z).into())
            .rotation((event.yaw, event.pitch).into());
        transform_event_writer.write(transform_event);
    }
}
