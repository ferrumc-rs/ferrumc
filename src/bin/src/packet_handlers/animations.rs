use ferrumc_macros::event_handler;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::outgoing::entity_animation::EntityAnimationEvent;
use ferrumc_net::utils::broadcast::{broadcast, BroadcastOptions};
use ferrumc_state::GlobalState;

#[event_handler]
async fn entity_animation(
    event: EntityAnimationEvent,
    state: GlobalState,
) -> Result<EntityAnimationEvent, NetError> {
    //TODO change this global broadcast to a broadcast that affects only players in the view distance
    //      of the player doing it, but as long as we still cant see other players, this will be fine.
    broadcast(
        &event.packet,
        &state,
        BroadcastOptions::default().except([event.entity]),
    )
        .await?;
    Ok(event)
}
