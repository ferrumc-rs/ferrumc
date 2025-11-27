use ferrumc_macros::packet;
use ferrumc_protocol::ids;

#[derive(Debug)]
#[packet(id = ids::CONFIGURATION_SERVERBOUND_CUSTOM_PAYLOAD, state = "configuration")]
pub struct ServerBoundPluginMessage {
    _channel: String,
    _data: Vec<u8>,
}
