use ferrumc_codec::enc::NetEncode;
use ferrumc_codec::network_types::varint::VarInt;
use ferrumc_macros::NetEncode;

#[derive(NetEncode)]
pub struct LoginPluginRequest {
    #[encode(default = VarInt::from(0x17))]
    pub packet_id: VarInt,
    pub channel: String,
    pub data: Vec<u8>,
}

impl LoginPluginRequest {
    pub fn new(channel: impl Into<String>, data: Vec<u8>) -> Self {
        Self::new_auto(channel.into(), data)
    }

    pub async fn server_brand(data: impl Into<String>) -> Self {
        let mut str_buffer = Vec::new();
        data.into().net_encode(&mut str_buffer).await.expect("tf");
        Self::new("minecraft:brand", str_buffer)
    }
}
