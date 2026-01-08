//! Use Item packet handler.
//!
//! Sent when the player uses the item currently in their hand (right-click in air).
//! This is used for eating food, throwing projectiles, etc.

use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::var_int::VarInt;

/// Which hand is being used for an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum Hand {
    /// Main hand (right hand by default)
    MainHand = 0,
    /// Off hand (left hand by default)
    OffHand = 1,
}

/// Sent when a player uses an item (right-click in air).
///
/// This is different from `use_item_on` which is for right-clicking blocks.
#[derive(NetDecode, Debug)]
#[packet(packet_id = "use_item", state = "play")]
pub struct UseItem {
    /// Which hand is being used
    pub hand: Hand,
    /// Sequence for acknowledgment
    pub sequence: VarInt,
    /// Player's yaw rotation
    pub yaw: f32,
    /// Player's pitch rotation
    pub pitch: f32,
}
