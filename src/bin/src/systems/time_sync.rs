//! World time synchronization system.
//!
//! This system manages the server's day/night cycle and broadcasts time updates
//! to all connected clients periodically.
//!
//! # Architecture
//!
//! The system runs on a dedicated schedule (once per second) to:
//! 1. Advance the world time by the elapsed ticks
//! 2. Broadcast `UpdateTime` packets to all connected players
//!
//! # Why Send Every Second?
//!
//! Although the `time_of_day_increasing` flag allows clients to advance time
//! locally, we still send periodic updates to:
//! - Correct any client/server drift
//! - Handle players who just joined
//! - Reflect `/time set` command changes
//!
//! Sending once per second (20 ticks) provides a good balance between
//! network efficiency and synchronization accuracy.

use bevy_ecs::prelude::{Query, Res};
use ferrumc_components::world_time::WorldTime;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::update_time::UpdateTime;
use std::sync::atomic::Ordering;
use tracing::trace;
use ferrumc_config::server_config::get_global_config;


/// Advances world time and broadcasts updates to all connected players.
///
/// This system:
/// 1. Advances the world age and time of day by `TICKS_PER_INVOCATION`
/// 2. Constructs an `UpdateTime` packet with the current state
/// 3. Sends the packet to all players with active connections
///
/// # System Parameters
///
/// - `world_time`: The shared world time resource
/// - `query`: All entities with a `StreamWriter` (connected players)
pub fn time_sync_system(world_time: Res<WorldTime>, query: Query<&StreamWriter>) {

    let tps = get_global_config().tps;

    // Advance world time
    world_time.tick(tps as i64);

    // Build the time packet
    let packet = UpdateTime::new(
        world_time.world_age(),
        world_time.time_of_day(),
        world_time.is_daylight_cycle_enabled(),
    );

    // Count successful sends for tracing
    let mut sent_count = 0;

    // Broadcast to all connected players
    for stream in query.iter() {
        // Skip dead connections
        if !stream.running.load(Ordering::Relaxed) {
            continue;
        }

        if stream.send_packet_ref(&packet).is_ok() {
            sent_count += 1;
        }
    }

    trace!(
        "Time sync: world_age={}, time_of_day={}, sent to {} players",
        packet.world_age,
        packet.time_of_day,
        sent_count
    );
}
