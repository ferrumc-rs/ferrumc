use ferrumc_macros::{packet, NetDecode};
use ferrumc_protocol::codec::net_types::network_position::NetworkPosition;
use ferrumc_protocol::ids;

/// Client-to-Server packet to request a "pick block" action.
#[derive(NetDecode, Debug)]
#[packet(id = ids::PLAY_SERVERBOUND_PICK_ITEM_FROM_BLOCK, state = "play")]
pub struct PickItemFromBlock {
    /// The location of the block the player is looking at.
    pub location: NetworkPosition,
    /// True if the client wants the block's NBT data (creative only)
    pub include_data: bool,
}
