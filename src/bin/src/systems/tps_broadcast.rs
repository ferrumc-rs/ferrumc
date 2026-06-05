//! Broadcasts the server's measured tick rate to players via the vanilla "Set Ticking State" packet.
//!
//! The client shows the received tick rate on its F3 debug screen, so streaming the real measured TPS
//! lets a developer watch server performance in-game. Sends are throttled to roughly once per second
//! both to avoid spamming the packet and to keep the client's tick pacing from jittering on every
//! small fluctuation.

use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::tick::TickCounter;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::set_ticking_state::SetTickingStatePacket;
use ferrumc_performance::ServerPerformance;
use std::time::Duration;
use tracing::warn;

/// Sends the measured TPS to every connected player about once per second.
pub fn broadcast_tps(
    tick: Res<TickCounter>,
    performance: Res<ServerPerformance>,
    players: Query<(Entity, &StreamWriter)>,
) {
    // Throttle to one broadcast per second (one per `target` ticks).
    let target = get_global_config().tps.max(1);
    if !tick.get().is_multiple_of(u64::from(target)) {
        return;
    }

    // Measured TPS over the last second; clamp to >= 1.0 so a momentarily stalled server never sends
    // a zero rate (which the client would treat as a frozen tick loop).
    let measured = performance.tps.tps(Duration::from_secs(1)).max(1.0);
    let packet = SetTickingStatePacket::new(measured, false);

    for (eid, writer) in players.iter() {
        writer.send_packet_ref(&packet).unwrap_or_else(|_| {
            warn!("Failed to send SetTickingStatePacket to player {eid}");
        });
    }
}
