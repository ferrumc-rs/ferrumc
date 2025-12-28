use bevy_ecs::message::{MessageReader, MessageWriter};
use bevy_ecs::prelude::Query;
use bevy_ecs::query::With;
use ferrumc_components::player::experience::Experience;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_messages::{PlayerGainedXP, PlayerLeveledUp};

pub fn player_gained_xp_handler(
    mut events: MessageReader<PlayerGainedXP>,
    mut player_leveled_up: MessageWriter<PlayerLeveledUp>,
    mut xp_players: Query<&mut Experience, With<PlayerIdentity>>
) {
    for event in events.read() {
        let Ok(mut xp) = xp_players.get_mut(event.player) else {continue};
        let old_level = xp.level;
        xp.total_xp += event.amount;
        let level = (xp.total_xp as f32 + 9.0).sqrt() - 3.0;
        xp.level = level.floor() as u32;
        if xp.level != old_level {
            player_leveled_up.write(PlayerLeveledUp {
                player: event.player,
                new_level: xp.level,
            });
        }
        xp.progress = level % 1.0;
    }
}