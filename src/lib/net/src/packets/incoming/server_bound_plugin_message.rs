use ferrumc_macros::packet;

#[derive(Debug)]
#[packet(packet_id = "custom_payload", state = "configuration")]
pub struct ServerBoundPluginMessage {
    _channel: String,
    _data: Vec<u8>,
}
