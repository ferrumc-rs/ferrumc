use crate::ids;
use ferrumc_macros::{NetEncode, packet};

/// Server-to-Client packet to set the player's selected hotbar slot.
#[derive(NetEncode, Copy, Clone)]
#[packet(id = ids::PLAY_CLIENTBOUND_SET_HELD_SLOT, state = "play")]
pub struct SetHeldItem {
    /// The hotbar slot to select (0-8).
    pub slot: u8,
}
