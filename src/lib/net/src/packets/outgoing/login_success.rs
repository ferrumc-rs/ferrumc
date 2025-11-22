use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;

#[derive(NetEncode)]
#[packet(packet_id = "login_finished", state = "login")]
pub struct LoginSuccessPacket<'a> {
    pub uuid: u128,
    pub username: &'a str,
    pub properties: LengthPrefixedVec<LoginSuccessProperties<'a>>,
}

impl<'a> LoginSuccessPacket<'a> {
    pub fn new(uuid: u128, username: &'a str) -> Self {
        Self {
            uuid,
            username,
            properties: LengthPrefixedVec::new(vec![]),
        }
    }
}

#[derive(NetEncode, Clone)]
pub struct LoginSuccessProperties<'a> {
    pub name: &'a str,
    pub value: &'a str,
    pub signature: PrefixedOptional<&'a str>,
}
