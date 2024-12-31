use tracing::debug;
use ferrumc_macros::event_handler;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::incoming::player_command::{PlayerCommandAction, PlayerDoActionEvent};
use ferrumc_net::packets::outgoing::entity_metadata::EntityMetadataPacket;
use ferrumc_net::utils::broadcast::broadcast;
use ferrumc_state::GlobalState;

#[event_handler]
async fn handle_player_do_action(
    event: PlayerDoActionEvent,
    state: GlobalState,
) -> Result<PlayerDoActionEvent, NetError> {
    debug!("player just did: {:?}", event.action);

    match event.action {
        PlayerCommandAction::StartSneaking => {
            let visual = EntityMetadataPacket::entity_sneaking_visual();
            let pressed = EntityMetadataPacket::entity_sneaking_pressed();

            broadcast(&visual, &state, Default::default()).await?;
            broadcast(&pressed, &state, Default::default()).await?;
        }
        _ => {}
    }

    Ok(event)
}