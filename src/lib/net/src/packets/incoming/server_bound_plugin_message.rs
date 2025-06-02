use ferrumc_macros::packet;
use ferrumc_net_codec::decode::NetDecode;

#[derive(Debug)]
#[packet(packet_id = "custom_payload", state = "configuration")]
pub struct ServerBoundPluginMessage {
    _channel: String,
    _data: Vec<u8>,
}
