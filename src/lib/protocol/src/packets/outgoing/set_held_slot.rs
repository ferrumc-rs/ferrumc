use ferrumc_macros::{packet, NetEncode};
use ferrumc_protocol::ids;

/// Server-to-Client packet to set the player's selected hotbar slot.
#[derive(NetEncode, Copy, Clone)]
#[packet(packet_id = "set_held_slot", state = "play")]
pub struct SetHeldItem {
    /// The hotbar slot to select (0-8).
    pub slot: u8,
}
