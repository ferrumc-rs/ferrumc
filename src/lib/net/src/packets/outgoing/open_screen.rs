use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_text::{TextComponent, TextComponentBuilder};
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = 0x33)]
pub struct OpenScreenPacket {
    pub window_id: VarInt,
    pub window_type: VarInt,
    pub window_title: TextComponent,
}

impl OpenScreenPacket {
    pub fn new(window_id: i32, window_type: VarInt, window_title: TextComponent) -> Self {
        Self {
            window_id: VarInt::new(window_id),
            window_type,
            window_title,
        }
    }

    pub fn with_empty_title(window_id: i32, window_type: VarInt) -> Self {
        Self::new(
            window_id,
            window_type,
            TextComponentBuilder::new("").build(),
        )
    }
}
