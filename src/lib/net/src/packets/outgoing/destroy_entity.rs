use ferrumc_net_codec::net_types::{var_int::VarInt, length_prefixed_vec::LengthPrefixedVec};
use ferrumc_macros::{packet, NetEncode};
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = 0x42)]
pub struct DestroyEntitiesPacket {
    pub entity_ids: LengthPrefixedVec<VarInt>,
}

impl DestroyEntitiesPacket {
    pub fn new(ids: Vec<usize>) -> Self {
        Self {
            entity_ids: LengthPrefixedVec::new(ids.into_iter().map(VarInt::from).collect()),
        }
    }
}
