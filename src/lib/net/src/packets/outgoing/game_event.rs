use ferrumc_macros::NetEncode;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetEncode)]
pub struct GameEventPacket {
    pub packet_id: VarInt, // 0x22
    
    pub event_id: u8,
    pub value: f32
}

impl GameEventPacket {
    pub fn new(event_id: u8, value: f32) -> Self {
        Self {
            packet_id: VarInt::new(0x22),
            event_id,
            value
        }
    }
    
    pub fn start_waiting_for_level_chunks() -> Self {
        Self::new(13, 0f32)
    }
}