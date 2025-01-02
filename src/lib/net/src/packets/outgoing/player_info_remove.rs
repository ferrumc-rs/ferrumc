use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = 0x3D)]
pub struct PlayerInfoRemovePacket {
    pub player_uuids: LengthPrefixedVec<u128>,
}

impl PlayerInfoRemovePacket {
    pub fn new(uuids: Vec<u128>) -> Self {
        Self {
            player_uuids: LengthPrefixedVec::new(uuids),
        }
    }
}
