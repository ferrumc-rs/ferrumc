use std::sync::Arc;
use ferrumc_macros::{event_handler, Constructor};
use tracing::info;
use crate::state::GlobalState;
use crate::utils::components::player::Player;

#[derive(Constructor)]
pub struct PlayerJoinWorldEvent {
    entity_id: u32,
}

#[event_handler]
async fn on_player_join_world(event: Arc<PlayerJoinWorldEvent>, state: GlobalState) {
    let Ok(player) = state.world.get_component::<Player>(event.entity_id).await else {
        tracing::warn!("Failed to get player with entity_id {}", event.entity_id);
        return;
    };

    info!("{} joined the world!", player.get_username());
}