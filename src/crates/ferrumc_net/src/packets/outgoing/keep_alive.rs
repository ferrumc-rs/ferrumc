use ferrumc_macros::Encode;

#[derive(Encode)]
pub struct KeepAlivePacketOut {
    #[encode(default = 0x23)]
    pub packet_id: i32,
    pub keep_alive_id: i64,
}

#[test]
fn test_auto_impl(){
    let packet = KeepAlivePacketOut::new_auto(99999i64);
    assert_eq!(packet.packet_id, 0x23);
    assert_eq!(packet.keep_alive_id, 99999i64);
}
