//! Use Item packet handler.
//!
//! Sent when the player uses the item currently in their hand (right-click in air).
//! This is used for eating food, throwing projectiles, etc.

use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::var_int::VarInt;

/// Sent when a player uses an item (right-click in air).
///
/// This is different from `use_item_on` which is for right-clicking blocks.
#[derive(NetDecode, Debug)]
#[packet(packet_id = "use_item", state = "play")]
pub struct UseItem {
    /// Which hand is being used (0 = main hand, 1 = off hand)
    pub hand: VarInt,
    /// Sequence for acknowledgment
    pub sequence: VarInt,
    /// Player's yaw rotation
    pub yaw: f32,
    /// Player's pitch rotation
    pub pitch: f32,
}
