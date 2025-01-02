use ferrumc_macros::event_handler;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::incoming::player_command::{PlayerCommandAction, PlayerDoActionEvent};
use ferrumc_net::packets::outgoing::entity_metadata::{EntityMetadata, EntityMetadataPacket};
use ferrumc_net::utils::broadcast::broadcast;
use ferrumc_state::GlobalState;
use tracing::trace;

#[event_handler]
async fn handle_player_do_action(
    event: PlayerDoActionEvent,
    state: GlobalState,
) -> Result<PlayerDoActionEvent, NetError> {
    trace!("player just did: {:?}", event.action);

    // TODO: replace this with a better system to support multiple actions
    match event.action {
        PlayerCommandAction::StartSneaking => {
            let packet = EntityMetadataPacket::new(
                event.entity_id,
                [
                    EntityMetadata::entity_sneaking_visual(),
                    EntityMetadata::entity_sneaking_pressed(),
                ],
            );

            broadcast(&packet, &state, Default::default()).await?;
        }
        PlayerCommandAction::StopSneaking => {
            let packet = EntityMetadataPacket::new(
                event.entity_id,
                [
                    EntityMetadata::entity_state_none(),
                    EntityMetadata::entity_standing(),
                ],
            );

            broadcast(&packet, &state, Default::default()).await?;
        }
        _ => {}
    }

    Ok(event)
}
