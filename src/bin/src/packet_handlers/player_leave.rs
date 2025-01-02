use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_macros::event_handler;
use ferrumc_net::connection::PlayerDisconnectEvent;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::outgoing::player_info_remove::PlayerInfoRemovePacket;
use ferrumc_net::packets::outgoing::remove_entities::RemoveEntitiesPacket;
use ferrumc_net::utils::broadcast::{broadcast, BroadcastOptions};
use ferrumc_state::GlobalState;
use tracing::info;

#[event_handler]
async fn handle_player_disconnect(
    event: PlayerDisconnectEvent,
    state: GlobalState,
) -> Result<PlayerDisconnectEvent, NetError> {
    let entity_id = event.entity_id;

    info!("Player disconnected: {:?}", entity_id);

    {
        let profile = state.universe.get::<PlayerIdentity>(entity_id)?;
        broadcast(
            &PlayerInfoRemovePacket::new(vec![profile.uuid]),
            &state,
            BroadcastOptions::default().all(),
        )
        .await?;
    }

    broadcast(
        &RemoveEntitiesPacket::from_entities([entity_id]),
        &state,
        BroadcastOptions::default().all(),
    )
    .await?;

    Ok(event)
}
