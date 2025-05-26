use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::Query;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::set_head_rotation::SetHeadRotationPacket;
use ferrumc_net::packets::packet_events::TransformEvent;
use ferrumc_net_codec::net_types::angle::NetAngle;
use tracing::error;

pub fn handle_player_move(
    mut events: EventReader<TransformEvent>,
    query: Query<(&Rotation, &PlayerIdentity)>,
    broadcast_query: Query<&StreamWriter>,
) {
    for event in events.read() {
        let entity = event.entity;

        let (rot, identity) = query.get(entity).unwrap();
        let head_rot_packet = SetHeadRotationPacket::new(
            identity.uuid as i32,
            NetAngle::from_degrees(rot.yaw as f64),
        );

        #[cfg(debug_assertions)]
        let start = std::time::Instant::now();

        for writer in broadcast_query.iter() {
            if !writer.running.load(std::sync::atomic::Ordering::Relaxed) {
                continue;
            }
            if let Err(err) = writer.send_packet(head_rot_packet.clone()) {
                error!("Failed to send head rotation packet: {:?}", err);
            }
        }

        #[cfg(debug_assertions)]
        tracing::trace!("broadcasting entity move took {:?}", start.elapsed());
    }
}
