use ferrumc_macros::{packet, NetDecode};

#[derive(NetDecode, Debug, Clone)]
#[packet(packet_id = "chat_command", state = "play")]
pub struct ChatCommandPacket {
    pub command: String,
}
