use bevy_ecs::prelude::{Entity, Query, Res, ResMut};
use ferrumc_core::conn::player_count_update_cooldown::PlayerCountUpdateCooldown;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_state::GlobalStateResource;

pub fn player_count_updater(
    state: Res<GlobalStateResource>,
    query: Query<(Entity, &PlayerIdentity)>,
    mut cooldown_tracker: ResMut<PlayerCountUpdateCooldown>,
) {
    // This list is more than likely to be updated on every join/leave event, but we do a manual
    // refresh every 10 seconds in case something desyncs
    if cooldown_tracker.last_update.elapsed().as_secs() < 10 {
        return;
    }
    state.0.players.player_list.clear();
    for (entity, player_identity) in query.iter() {
        let uuid = player_identity.uuid;
        let username = &player_identity.username;
        state
            .0
            .players
            .player_list
            .insert(entity, (uuid.as_u128(), username.clone()));
    }
    cooldown_tracker.last_update = std::time::Instant::now();
}
