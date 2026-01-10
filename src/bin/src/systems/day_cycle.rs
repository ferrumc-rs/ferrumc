use bevy_ecs::prelude::{Entity, Query, ResMut, Resource};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::update_time::UpdateTimePacket;
use tracing::debug;
use ferrumc_core::time::WorldTime;

pub fn tick_daylight_cycle(
    mut world_time: ResMut<WorldTime>,
    players: Query<(Entity, &StreamWriter)>,
) {
    world_time.advance_tick();

    // Send time update every 20 ticks
    if world_time.current_time().is_multiple_of(20) {
        let packet = UpdateTimePacket {
            world_age: 0,
            time_of_day: world_time.current_time() as _,
            time_of_day_increasing: true,
        };

        for (id, player) in players.iter() {
            player.send_packet_ref(&packet).unwrap_or_else(|err| {
                debug!("Failed to send update time packet to player {id}: {err}")
            });
        }
    }
}
