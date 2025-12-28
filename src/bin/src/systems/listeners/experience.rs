use bevy_ecs::message::{MessageReader, MessageWriter};
use bevy_ecs::prelude::Query;
use bevy_ecs::query::With;
use tracing::error;
use ferrumc_components::player::experience::Experience;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_messages::{PlayerGainedXP, PlayerLeveledUp};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::set_experience::SetExperience;
use ferrumc_net_codec::net_types::var_int::VarInt;

pub fn player_gained_xp_handler(
    mut events: MessageReader<PlayerGainedXP>,
    mut player_leveled_up: MessageWriter<PlayerLeveledUp>,
    mut xp_players: Query<(&mut Experience, &StreamWriter), With<PlayerIdentity>>
) {
    for event in events.read() {
        let Ok(player) = xp_players.get_mut(event.player) else {continue};
        let mut xp = player.0;
        let writer = player.1;
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

        let packet = SetExperience {
            level: VarInt(xp.level as i32),
            experience_bar: xp.progress,
            total_experience: VarInt(xp.total_xp as i32),
        };
        if let Err(err) = writer.send_packet_ref(&packet) {
            error!("Failed to send set_experience packet: {:?}", err);
        };
    }
}