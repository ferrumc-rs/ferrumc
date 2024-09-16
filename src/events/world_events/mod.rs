use crate::state::GlobalState;
use crate::utils::components::player::{Player};
use ferrumc_macros::{event_handler, Constructor};
use std::sync::Arc;
use tracing::{error, info};

#[derive(Constructor)]
pub struct PlayerJoinWorldEvent {
    entity_id: usize,
}

#[event_handler(priority = "slow")]
async fn on_player_join_world(event: Arc<PlayerJoinWorldEvent>, state: GlobalState) {
    if let Err(e) = send_join_message(event.entity_id, state).await {
        error!("Failed to send join message: {:?}", e);
    }
}

async fn send_join_message(entity_id: usize, state: GlobalState) -> crate::Result<()> {
    let player = state.world.get_component::<Player>(entity_id).await?;
    
    info!("{} joined the world!", player.get_username());
    
    Ok(())
}