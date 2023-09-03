use async_trait::async_trait;
use tokio::net::TcpStream;
use crate::network::packet::InboundPacket;

pub struct PacketPlayInLoginStart {
    pub username: String,
}


#[async_trait]
impl InboundPacket for PacketPlayInLoginStart {
    async fn deserialize(bytes: Vec<u8>) -> Result<Self, anyhow::Error> where Self: Sized {
        println!("data: {:?}", bytes);

        Ok(Self {
            username: "test".to_string()
        })
    }

    fn get_id(&self) -> u32 {
        0x00
    }

    async fn handle(&self, stream: &mut TcpStream) {
        println!("data: {:?}", self.username)
    }
}


// #[async_trait]
// impl Packet for PacketPlayInLoginStart {
//     fn serialize(&self) -> Vec<u8> { todo!() }
//
//     fn deserialize(bytes: Vec<u8>) -> Result<Self, Error> where Self: Sized { todo!() }
//
//     fn get_id(&self) -> u32 {
//         todo!()
//     }
//
//     fn get_name(&self) -> String {
//         todo!()
//     }
//
//     async fn handle(&self, stream: &mut TcpStream) {
//         todo!()
//     }
// }