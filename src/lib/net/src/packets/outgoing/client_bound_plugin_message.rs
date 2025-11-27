use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};

const SERVER_VERSION: &str = concat!("FerrumC v", env!("FERRUMC_VERSION"), env!("BUILD_TYPE"));

#[derive(NetEncode, Clone)]
#[packet(packet_id = "custom_payload", state = "configuration")]
pub struct ClientBoundPluginMessagePacket {
    pub channel: String,
    pub data: Vec<u8>,
}

impl ClientBoundPluginMessagePacket {
    pub fn brand() -> ClientBoundPluginMessagePacket {
        let mut data = Vec::<u8>::new();
        SERVER_VERSION
            .encode(&mut data, &NetEncodeOpts::None)
            .expect("failed to write brand name to buffer");
        Self {
            channel: "minecraft:brand".to_string(),
            data,
        }
    }
}
