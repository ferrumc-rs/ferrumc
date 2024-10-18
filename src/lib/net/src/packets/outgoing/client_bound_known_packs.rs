use ferrumc_macros::NetEncode;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetEncode)]
pub struct ClientBoundKnownPacksPacket<'a> {
    // 0x0E @ "configuration"
    pub packet_id: VarInt,
    pub packs: LengthPrefixedVec<Pack<'a>>,
}

#[derive(NetEncode)]
pub struct Pack<'a> {
    pub namespace: &'a str,
    pub id: &'a str,
    pub version: &'a str,
}

impl<'a> Default for ClientBoundKnownPacksPacket<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ClientBoundKnownPacksPacket<'a> {
    pub fn new() -> Self {
        Self {
            packet_id: VarInt::from(0x0E),
            packs: LengthPrefixedVec::new(vec![
                Pack::new("minecraft:core", "base", "1.21"),
            ]),
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