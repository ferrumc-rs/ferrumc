use bevy_ecs::prelude::{Commands, Entity, Query, ResMut};
use ferrumc_core::time::{LastSentTimeUpdate, WorldTime};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::update_time::UpdateTimePacket;
use tracing::{debug, warn};

pub fn tick_daylight_cycle(
    mut world_time: ResMut<WorldTime>,
    players: Query<(Entity, &StreamWriter)>,
    mut last_sent_time: Query<&mut LastSentTimeUpdate>,
    mut commands: Commands,
) {
    world_time.advance_tick();

    let packet = UpdateTimePacket {
        world_age: 0,
        time_of_day: world_time.current_time() as _,
        time_of_day_increasing: true,
    };

    for (eid, writer) in players.iter() {
        if let Ok(mut last_sent_time_update) = last_sent_time.get_mut(eid) {
            if last_sent_time_update.should_resend() {
                last_sent_time_update.reset();

                writer.send_packet_ref(&packet).unwrap_or_else(|_| {
                    warn!("Failed to send UpdateTimePacket to player {}", eid);
                });
            }
        } else {
            commands.entity(eid).insert(LastSentTimeUpdate::default());

            writer.send_packet_ref(&packet).unwrap_or_else(|_| {
                warn!("Failed to send UpdateTimePacket to player {}", eid);
            });
        }
    }

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
