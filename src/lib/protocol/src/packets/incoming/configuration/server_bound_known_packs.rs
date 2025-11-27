use ferrumc_macros::{packet, NetDecode};
use ferrumc_protocol::codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_protocol::ids;

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
