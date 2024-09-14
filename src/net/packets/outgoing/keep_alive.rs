use ferrumc_codec::network_types::varint::VarInt;

use ferrumc_macros::NetEncode;

use crate::utils::components::keep_alive::KeepAlive;

#[derive(NetEncode, Debug)]
pub struct KeepAlivePacketOut {
    #[encode(default = VarInt::from(0x23))]
    pub packet_id: VarInt,
    pub keep_alive_id: i64,
}

#[test]
fn test_auto_impl() {
    let packet = KeepAlivePacketOut::new_auto(99999i64);
    assert_eq!(packet.packet_id.get_val(), 0x23);
    assert_eq!(packet.keep_alive_id, 99999i64);
}

impl From<&mut KeepAlive> for KeepAlivePacketOut {
    fn from(val: &mut KeepAlive) -> Self {
        KeepAlivePacketOut::new_auto(val.data)
    }
}
