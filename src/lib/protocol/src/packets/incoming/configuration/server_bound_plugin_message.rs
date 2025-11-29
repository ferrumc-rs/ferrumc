use ferrumc_macros::packet;

#[derive(Debug)]
#[packet(id = ids::CONFIGURATION_SERVERBOUND_CUSTOM_PAYLOAD, state = "configuration")]
pub struct ServerBoundPluginMessage {
    _channel: String,
    _data: Vec<u8>,
}
