use crate::codec::net_types::network_position::NetworkPosition;
use ferrumc_macros::{NetDecode, packet};

/// Client-to-Server packet to request a "pick block" action.
#[derive(NetDecode, Debug)]
#[packet(id = ids::PLAY_SERVERBOUND_PICK_ITEM_FROM_BLOCK, state = "play")]
pub struct PickItemFromBlock {
    /// The location of the block the player is looking at.
    pub location: NetworkPosition,
    /// True if the client wants the block's NBT data (creative only)
    pub include_data: bool,
}
