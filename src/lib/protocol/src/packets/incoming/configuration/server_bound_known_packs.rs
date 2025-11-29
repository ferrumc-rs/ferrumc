use crate::codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use crate::ids;
use ferrumc_macros::{NetDecode, packet};

#[derive(Debug, NetDecode)]
#[packet(id = ids::CONFIGURATION_SERVERBOUND_SELECT_KNOWN_PACKS, state = "configuration")]
pub struct ServerBoundKnownPacks {
    pub packs: LengthPrefixedVec<PackOwned>,
}

#[derive(Debug, NetDecode)]
#[expect(dead_code)]
pub struct PackOwned {
    namespace: String,
    id: String,
    version: String,
}
