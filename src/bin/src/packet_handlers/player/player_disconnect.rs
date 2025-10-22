use bevy_ecs::{event::EventReader, prelude::Res, system::Commands};
use ferrumc_core::conn::player_disconnect_event::PlayerDisconnectEvent;
use ferrumc_state::GlobalStateResource;

pub fn handle(
    mut cmd: Commands,
    mut events: EventReader<PlayerDisconnectEvent>,
    state: Res<GlobalStateResource>,
) {
    for event in events.read() {
        let uuid = event.identity.uuid.as_u128();
        let username = &event.identity.username;
        if let Err(e) = state.0.world.save_player_state(uuid, &event.data) {
            tracing::error!("Failed to save player state for {}: {}", username, e);
        } else {
            tracing::info!("Player state saved for {}", username);
        }
        cmd.entity(event.entity).despawn();
    }
}
