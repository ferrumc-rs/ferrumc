//! Packet to remove players from the tab list.
//!
//! Sent when a player disconnects to remove them from other players' tab lists.

use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;

/// Packet to remove players from the tab list
#[derive(NetEncode)]
#[packet(packet_id = "player_info_remove", state = "play")]
pub struct PlayerInfoRemovePacket {
    pub uuids: LengthPrefixedVec<u128>,
}

impl PlayerInfoRemovePacket {
    /// Create a packet to remove multiple players
    pub fn new(uuids: Vec<u128>) -> Self {
        Self {
            uuids: LengthPrefixedVec::new(uuids),
        }
    }

    /// Create a packet to remove a single player
    pub fn single(uuid: u128) -> Self {
        Self::new(vec![uuid])
    }
}
