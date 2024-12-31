use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_macros::event_handler;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::outgoing::teleport_entity::TeleportEntityPacket;
use ferrumc_net::packets::packet_events::TransformEvent;
use ferrumc_net::utils::broadcast::{broadcast, BroadcastOptions};
use ferrumc_net::utils::ecs_helpers::EntityExt;
use ferrumc_state::GlobalState;

#[event_handler(priority = "normal")]
async fn handle_player_move(
    event: TransformEvent,
    state: GlobalState,
) -> Result<TransformEvent, NetError> {
    let entity = event.conn_id;

    let pos = entity.get::<Position>(&state)?;
    let rot = entity.get::<Rotation>(&state)?;
    let grounded = entity.get::<OnGround>(&state)?;

    let packet = TeleportEntityPacket::new(
        entity,
        &pos,
        &rot,
        grounded.0
    );
    
    let start = std::time::Instant::now();
    broadcast(&packet, &state, BroadcastOptions::default().all()).await?;
    tracing::debug!("broadcasting teleport took {:?}", start.elapsed());


    Ok(event)
}