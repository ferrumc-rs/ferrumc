use async_trait::async_trait;
use tokio::net::TcpStream;
use crate::network::packet::Packet;
use crate::utils::{write_varlong, write_varint};

pub struct PacketPlayOutPong {
    pub payload: i64
}

#[async_trait]
impl Packet for PacketPlayOutPong {
    fn serialize(&self) -> Vec<u8> {
        todo!()
        //     Not used, but required by the trait, Should probably fix it 💀
    }

    fn deserialize(_bytes: Vec<u8>) -> Result<Self, anyhow::Error> where Self: Sized {
        todo!()
    }

    fn get_id(&self) -> u32 {
        0x00
    }

    fn get_name(&self) -> String {
        todo!()
    }

    async fn handle(&self, _stream: &mut TcpStream) {
        todo!()
    }
}

impl PacketPlayOutPong {
    pub async fn specialized_serialize(&self) -> Result<Vec<u8>, anyhow::Error> {

        let mut temp_buffer = Vec::new();

        write_varint(&mut temp_buffer, 0x09).await; // packet id + length of LONG

        write_varint(&mut temp_buffer, 0x01).await;

        write_varlong(&mut temp_buffer, self.payload).await?;

        let _packet_length = temp_buffer.len() as i32;

        let mut final_buffer = Vec::new();

        final_buffer.extend_from_slice(&temp_buffer);

        Ok(final_buffer)
    }
}