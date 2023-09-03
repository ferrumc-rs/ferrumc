use std::io::{Cursor};
use anyhow::Error;
use async_trait::async_trait;
use tokio::io::{AsyncWriteExt};
use tokio::net::TcpStream;
use crate::network::packet::{InboundPacket, OutboundPacket};
use crate::utils::{read_varint, read_varlong};
use crate::network::packet::outbound::packet_play_out_pong::PacketPlayOutPong;

pub struct PacketPlayInPing{
    pub payload: i64

}

#[async_trait]
impl InboundPacket for PacketPlayInPing {
    async fn deserialize(bytes: Vec<u8>) -> Result<Self, Error> where Self: Sized {
        let mut cursor = Cursor::new(bytes);

        let _packet_length = read_varint(&mut cursor)?;

        // Read Packet ID (should be 1 for a Ping packet)
        let packet_id = read_varint(&mut cursor)?;

        if packet_id != 1 {
            return Err(Error::msg("Invalid packet ID"));
        }

        // Read payload
        let payload = read_varlong(&mut cursor)?;
        println!("data: {:?}", payload);

        Ok(Self {
            payload
        })
    }

    fn get_id(&self) -> u32 {
        0x01
    }

    async fn handle(&self, stream: &mut TcpStream) {
        let pong_packet = PacketPlayOutPong {
            payload: self.payload
        };
        if let Ok(e) = pong_packet.serialize().await {
            if let Err(e) = stream.write_all((&e).as_ref()).await {
                println!("There was an error sending the pong packet: {:?}", e);
            }
        }else{
            println!("There was an error serializing the pong packet");
        }

        if let Err(_) = stream.flush().await {
            // println!("There was an error flushing the stream: {:?}", e);
        }

        if let Err(_) = stream.shutdown().await {
            // println!("There was an error shutting down the stream: {:?}", e);
        }
    }
}

//
// #[async_trait]
// impl Packet for PacketPlayInPing {
//     fn serialize(&self) -> Vec<u8> {
//         todo!()
//     }
//
//     fn deserialize(data: Vec<u8>) -> Result<Self, Error> where Self: Sized {
//         let mut cursor = Cursor::new(data);
//
//         let _packet_length = read_varint(&mut cursor)?;
//
//         // Read Packet ID (should be 1 for a Ping packet)
//         let packet_id = read_varint(&mut cursor)?;
//         if packet_id != 1 {
//             return Err(Error::msg("Invalid packet ID")); // Not possible??
//         }
//
//         // Read payload
//         let payload = read_varlong(&mut cursor)?;
//         println!("data: {:?}", payload);
//
//         Ok(Self {
//             payload
//         })
//     }
//
//
//     fn get_id(&self) -> u32 {
//         0x01
//     }
//
//     fn get_name(&self) -> String {
//         todo!()
//     }
//
//     async fn handle(&self, stream: &mut TcpStream) {
//
//
//         let pong_packed = PacketPlayOutPong {
//             payload: self.payload
//         };
//
//         let pong = pong_packed.specialized_serialize().await.unwrap();
//
//         stream.write_all((&pong).as_ref()).await.unwrap();
//         println!("Sent pong packet with payload: {:?}", pong);
//
//         // stream.shutdown().
//         // dont unwrap, handle error too
//         if let Ok(_) = stream.shutdown().await {
//             println!("Successfully closed connection");
//         }
//     }
// }