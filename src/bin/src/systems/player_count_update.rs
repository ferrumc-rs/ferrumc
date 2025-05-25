use bevy_ecs::prelude::{EventReader, Query, Res};
use ferrumc_core::conn::force_player_recount_event::ForcePlayerRecountEvent;
use ferrumc_core::conn::player_count_update_cooldown::PlayerCountUpdateCooldown;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_state::GlobalStateResource;
use tracing::trace;

pub fn player_count_updater(
    state: Res<GlobalStateResource>,
    query: Query<&PlayerIdentity>,
    cooldown_tracker: Res<PlayerCountUpdateCooldown>,
    mut force_events: EventReader<ForcePlayerRecountEvent>,
) {
    if cooldown_tracker.last_update.elapsed().as_secs() < 5 {
        if force_events.is_empty() {
            // If the cooldown is still active and no force recount events, skip this update
            return;
        }
        // Drain the force recount events
        force_events
            .read()
            .for_each(|_| trace!("Force recount event received"))
    }
    state.0.players.clear();
    for player_identity in query.iter() {
        let uuid = player_identity.uuid;
        let username = &player_identity.username;
        state.0.players.insert(uuid, username.clone());
    }
}
