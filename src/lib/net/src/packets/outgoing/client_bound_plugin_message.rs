use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::{
    encode::NetEncode,
    net_types::var_int::VarInt
};
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = 0x01)]
pub struct ConfigurationPluginMessagePacket<T>
where
    T: NetEncode {
    pub channel: String,
    pub data: T,
}

#[derive(NetEncode)]
#[packet(packet_id = 0x19)]
pub struct PlayPluginMessagePacket<T>
where
    T: NetEncode {
    pub channel: String,
    pub data: T,
}

#[derive(NetEncode, Clone)]
#[packet(packet_id = 0x04)]
pub struct LoginPluginMessagePacket<T>
where
    T: NetEncode,
{
    pub message_id: VarInt,
    pub channel: String,
    pub data: T,
}

impl<T> ConfigurationPluginMessagePacket<T>
where
    T: NetEncode {
    pub fn new(channel: String, data: T) -> Self
    {
        Self {
            channel,
            data
        }
    }
}

impl<T> PlayPluginMessagePacket<T>
where
    T: NetEncode,
{
    pub fn new(channel: String, data: T) -> Self
    {
        Self {
            channel,
            data
        }
    }
}

impl<T> LoginPluginMessagePacket<T>
where
    T: NetEncode,
{
    pub fn new(id: u32, channel: String, data: T) -> Self
    {
        Self {
            message_id: VarInt::new(id as i32),
            channel,
            data
        }
    }
}
