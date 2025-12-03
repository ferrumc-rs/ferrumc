use bevy_ecs::prelude::{Entity, Query, Res, ResMut};
use ferrumc_components::conn::player_count_update_cooldown::PlayerCountUpdateCooldown;
use ferrumc_components::player::identity::PlayerIdentity;
use ferrumc_components::state::server_state::GlobalStateResource;

pub fn player_count_updater(
    state: Res<GlobalStateResource>,
    query: Query<(Entity, &PlayerIdentity)>,
    mut cooldown_tracker: ResMut<PlayerCountUpdateCooldown>,
) {
    // Frequency is controlled by the schedule period.
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
