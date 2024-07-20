use ferrumc_macros::{Decode, Encode};
use ferrumc_utils::encoding::varint::VarInt;
use ferrumc_utils::encoding::varlong::Varlong;

#[derive(Encode, Debug)]
pub struct KeepAlivePacketOut {
    #[encode(default = VarInt::from(0x23))]
    pub packet_id: VarInt,
    pub keep_alive_id: i64,
}

#[test]
fn test_auto_impl(){
    let packet = KeepAlivePacketOut::new_auto(99999i64);
    assert_eq!(packet.packet_id, 0x23);
    assert_eq!(packet.keep_alive_id, 99999i64);
}
