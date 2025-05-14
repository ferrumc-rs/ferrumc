use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::Query;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_macros::event_handler;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::outgoing::set_head_rotation::SetHeadRotationPacket;
use ferrumc_net::packets::packet_events::TransformEvent;
use ferrumc_net::utils::broadcast::{broadcast, BroadcastOptions};
use ferrumc_net::utils::ecs_helpers::EntityExt;
use ferrumc_net_codec::net_types::angle::NetAngle;
use ferrumc_state::GlobalState;

pub fn handle_player_move(
    mut events: EventReader<TransformEvent>,
    query: Query<(&Rotation, &PlayerIdentity)>,
    broadcast_query: Query<&StreamWriter>,
) {
    for event in events.read() {
        let entity = event.entity;

        let (rot, identity) = query.get(entity).unwrap();
        let head_rot_packet =
            SetHeadRotationPacket::new(identity.uuid as i32, NetAngle::from_degrees(rot.yaw as f64));

        let start = std::time::Instant::now();
        for writer in broadcast_query.iter() {
            writer.write_packet(&head_rot_packet)?;
        }

        tracing::trace!("broadcasting entity move took {:?}", start.elapsed());
    }
}
