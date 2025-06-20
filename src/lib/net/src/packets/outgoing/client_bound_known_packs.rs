use ferrumc_macros::{NetEncode, packet};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "select_known_packs", state = "configuration")]
pub struct ClientBoundKnownPacksPacket<'a> {
    pub packs: LengthPrefixedVec<Pack<'a>>,
}

#[derive(NetEncode)]
pub struct Pack<'a> {
    pub namespace: &'a str,
    pub id: &'a str,
    pub version: &'a str,
}

impl Default for ClientBoundKnownPacksPacket<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientBoundKnownPacksPacket<'_> {
    pub fn new() -> Self {
        Self {
            packs: LengthPrefixedVec::new(vec![Pack::new("minecraft", "core", "1.21")]),
        }
    }
}

impl<'a> Pack<'a> {
    pub fn new(namespace: &'a str, id: &'a str, version: &'a str) -> Self {
        Self {
            namespace,
            id,
            version,
        }
    }
}
