use bevy_ecs::prelude::{Entity, Query, ResMut, Resource};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::update_time::UpdateTimePacket;
use tracing::debug;

#[derive(Resource, Debug, Default)]
/// Holds the current world time. The minimum value is 0 and the maximum is 23999 (24000 rolls back over)
///
/// Day is from tick 0..12000
/// Dusk is from tick 12000..13000
/// Night is from tick 13000..23000
/// Dawn is from tick 23000..24000
pub struct WorldTime(pub u16);

pub fn tick_daylight_cycle(
    mut world_time: ResMut<WorldTime>,
    players: Query<(Entity, &StreamWriter)>,
) {
    world_time.0 = (world_time.0 + 1) % 24000;

    // Send time update every 20 ticks
    if world_time.0.is_multiple_of(20) {
        let packet = UpdateTimePacket {
            world_age: 0,
            time_of_day: world_time.0 as _,
            time_of_day_increasing: true,
        };

        for (id, player) in players.iter() {
            player.send_packet_ref(&packet).unwrap_or_else(|err| {
                debug!("Failed to send update time packet to player {id}: {err}")
            });
        }
    }
}
