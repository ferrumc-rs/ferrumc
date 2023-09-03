use std::io::Cursor;
use std::sync::Arc;

use anyhow::{Error, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use crate::network::connection::state::ConnectionState;
use crate::network::packet::{OutboundPacket, REGISTRY};
use crate::utils::{read_varint, read_varint_async};

pub struct Player {
    pub uuid: String,
    pub username: String,
    pub connection: Arc<Mutex<Connection>>,  // Wrapped in Mutex for mutable access
}

impl Player {
    pub fn new(stream: TcpStream) -> Self {
        Player {
            uuid: "".to_string(),
            username: "".to_string(),
            connection: Arc::new(Mutex::new(Connection::new(ConnectionState::Handshaking, stream))),
        }
    }

    pub async fn init(&self) -> Result<()> {
        let mut connection = self.connection.lock().await;
        connection.start_connection().await?;
        Ok(())
    }
}


pub struct Connection {
    pub state: ConnectionState,
    pub stream: TcpStream,
}

impl Connection {
    pub fn new(state: ConnectionState, stream: TcpStream) -> Self {
        Connection {
            state,
            stream,
        }
    }
    pub async fn send_packet(&mut self, packet: impl OutboundPacket) -> Result<()> {
        let serialized = packet.serialize().await?;

        self.stream.write_all(serialized.as_ref()).await?;

        Ok(())
    }

    pub async fn start_connection(&mut self) -> Result<()> {
        loop {

            let packet_len = self.stream.read_u8().await?;

            println!("Packet Length: {:?}", packet_len);

            // let cursor = &mut Cursor::new(&mut self.stream.read);
            //
            // let packet_len_tset = read_varint(cursor).await?;
            // println!(" test :  :  Packet Length: {:?}", packet_len_tset);
            //
            // let packet_len = read_varint_async(&mut self.stream).await?;
            //
            // println!("Packet Length: {:?}", packet_len);
            //
            // let mut packet_data = vec![0; packet_len as usize];
            // self.stream.read_exact(&mut packet_data).await?;
            //
            // println!("Packet Data: {:?}", packet_data);
            //
            // if let Some(packet) = REGISTRY.deserialize_inbound(self.state, packet_data).await {
            //     println!("Packet Id: {:?}", packet.get_id());
            //     packet.handle(&mut self.stream).await;
            // }
        }

        // let mut buf = [0; 32];  // Made buffer to 32 bytes
        //
        //
        //
        // loop {
        //     let n = match self.stream.read(&mut buf).await {
        //         Ok(n) if n == 0 => return Ok(()),
        //         Ok(n) => n,
        //         Err(_e) => return Err(Error::msg("failed to read from socket")),
        //     };
        //
        //     let packet_data = Vec::from(&buf[0..n]);
        //
        //     println!("Received packet: {:?}", packet_data);
        //
        //     if let Some(packet) = REGISTRY.deserialize_inbound(self.state, packet_data).await {
        //         println!("Packet Id: {:?}", packet.get_id());
        //         packet.handle(&mut self.stream).await;
        //     }
        // }
    }
}