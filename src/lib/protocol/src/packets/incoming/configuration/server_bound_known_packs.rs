use crate::codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_core::resources::data_pack::DataPackEntry;
use ferrumc_macros::{NetDecode, packet};

#[derive(Debug, NetDecode)]
#[packet(id = ids::CONFIGURATION_SERVERBOUND_SELECT_KNOWN_PACKS, state = "configuration")]
pub struct ServerBoundKnownPacket {
    pub packs: LengthPrefixedVec<DataPackEntry>,
}
