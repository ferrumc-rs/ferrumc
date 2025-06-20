use ferrumc_macros::{NetEncode, packet};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use std::io::Write;

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
    pub signature: Option<&'a str>,
}
