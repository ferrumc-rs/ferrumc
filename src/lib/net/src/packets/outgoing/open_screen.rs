use ferrumc_macros::{packet, NetEncode};
use ferrumc_nbt::NBT;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_text::TextComponent;

#[derive(NetEncode)]
#[packet(packet_id = "open_screen", state = "play")]
pub struct OpenScreen {
    pub window_id: VarInt,
    pub window_type: VarInt,
    pub title: NBT<TextComponent>,
}
