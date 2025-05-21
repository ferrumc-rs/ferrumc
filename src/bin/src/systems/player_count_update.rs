use bevy_ecs::prelude::{Query, Res};
use ferrumc_core::conn::player_count_update_cooldown::PlayerCountUpdateCooldown;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_state::GlobalStateResource;

pub fn player_count_updater(
    state: Res<GlobalStateResource>,
    query: Query<&PlayerIdentity>,
    cooldown_tracker: Res<PlayerCountUpdateCooldown>,
) {
    if cooldown_tracker.last_update.elapsed().as_secs() < 5 {
        return;
    }
    state.0.players.clear();
    for player_identity in query.iter() {
        let uuid = player_identity.uuid;
        let username = &player_identity.username;
        state.0.players.insert(uuid, username.clone());
    }
}
