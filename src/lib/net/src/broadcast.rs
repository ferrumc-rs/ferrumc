//! Utilities for broadcasting packets to multiple players.
//!
//! This module provides helper functions for common broadcast patterns,
//! reducing code duplication across packet handlers.

use bevy_ecs::prelude::Entity;
use ferrumc_net_codec::encode::NetEncode;
use tracing::error;

use crate::connection::StreamWriter;

/// Broadcasts a packet to all players except the sender.
///
/// # Arguments
/// * `sender` - The entity ID of the sender (will be excluded from broadcast)
/// * `packet` - The packet to broadcast (must implement NetEncode)
/// * `recipients` - Iterator of (Entity, &StreamWriter) pairs to broadcast to
///
/// # Example
/// ```ignore
/// broadcast_packet_except(
///     sender_entity,
///     &my_packet,
///     conn_query.iter(),
/// );
/// ```
pub fn broadcast_packet_except<'a, P, I>(sender: Entity, packet: &P, recipients: I)
where
    P: NetEncode + Send,
    I: Iterator<Item = (Entity, &'a StreamWriter)>,
{
    for (entity, writer) in recipients {
        if entity == sender {
            continue;
        }
        if !writer.is_running() {
            continue;
        }
        if let Err(err) = writer.send_packet_ref(packet) {
            error!("Failed to broadcast packet to {:?}: {:?}", entity, err);
        }
    }
}

/// Broadcasts a packet to all connected players.
///
/// # Arguments
/// * `packet` - The packet to broadcast (must implement NetEncode)
/// * `recipients` - Iterator of (Entity, &StreamWriter) pairs to broadcast to
pub fn broadcast_packet_all<'a, P, I>(packet: &P, recipients: I)
where
    P: NetEncode + Send,
    I: Iterator<Item = (Entity, &'a StreamWriter)>,
{
    for (entity, writer) in recipients {
        if !writer.is_running() {
            continue;
        }
        if let Err(err) = writer.send_packet_ref(packet) {
            error!("Failed to broadcast packet to {:?}: {:?}", entity, err);
        }
    }
}
