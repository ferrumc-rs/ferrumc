use ferrumc_core::transform::rotation::Rotation;
use ferrumc_macros::event_handler;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::outgoing::set_head_rotation::SetHeadRotationPacket;
use ferrumc_net::packets::packet_events::TransformEvent;
use ferrumc_net::utils::broadcast::{broadcast, BroadcastOptions};
use ferrumc_net::utils::ecs_helpers::EntityExt;
use ferrumc_net_codec::net_types::angle::NetAngle;
use ferrumc_state::GlobalState;

#[event_handler(priority = "normal")]
async fn handle_player_move(
    event: TransformEvent,
    state: GlobalState,
) -> Result<TransformEvent, NetError> {
    let entity = event.conn_id;

    // let pos = entity.get::<Position>(&state)?;
    let rot = entity.get::<Rotation>(&state)?;
    // let grounded = entity.get::<OnGround>(&state)?;

    // let teleport_packet = TeleportEntityPacket::new(entity, &pos, &rot, grounded.0);
    let head_rot_packet =
        SetHeadRotationPacket::new(entity as i32, NetAngle::from_degrees(rot.yaw as f64));

    let start = std::time::Instant::now();
    // broadcast(&teleport_packet, &state, BroadcastOptions::default().all()).await?;
    broadcast(&head_rot_packet, &state, BroadcastOptions::default().all()).await?;

    tracing::trace!("broadcasting entity move took {:?}", start.elapsed());

    Ok(event)
}
