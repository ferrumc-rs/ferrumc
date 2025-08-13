use tracing::error;

use bevy_ecs::system::{Query, ResMut};
use ferrumc_core::time::WorldTime;
use ferrumc_net::{connection::StreamWriter, packets::outgoing::update_time::UpdateTimePacket};

pub fn day_night_cycle_system(players: Query<&StreamWriter>, mut world_time: ResMut<WorldTime>) {
    world_time.world_age += 1;
    world_time.tick_counter += 1;

    world_time.time_of_day += 1;
    if world_time.time_of_day >= 24000 {
        world_time.time_of_day = 0;
    }

    // Broadcast the updated time to all online players
    // This will send an update every second (20 ticks)
    if world_time.tick_counter.is_multiple_of(20) {
        let packet = UpdateTimePacket {
            world_age: world_time.world_age,
            time_of_day: world_time.time_of_day,
            time_of_day_increasing: true,
        };

        for player in players.iter() {
            if let Err(e) = player.send_packet(packet) {
                error!("Failed to send time update: {:?}", e);
            }
        }
    }
}
