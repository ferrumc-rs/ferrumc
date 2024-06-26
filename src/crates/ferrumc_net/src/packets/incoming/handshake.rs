use std::sync::Arc;
use log::{debug, info};
use ferrumc_macros::{Decode, packet};
use ferrumc_utils::encoding::varint::VarInt;
use crate::{Connection, State};
use crate::packets::IncomingPacket;

#[derive(Decode)]
#[packet(packet_id = 0x00, state = "handshake")]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: VarInt,
}

impl IncomingPacket for Handshake {
    async fn handle(&self, conn: &mut Arc<tokio::sync::RwLock<Connection>>) -> Result<(), Error> {
        info!("Handling handshake packet");

        debug!("Obtaining connection lock for protocol version and state");

        let mut conn_write = conn.write().await;
        
        debug!("Successfully obtained connection lock for protocol version and state");
        
        conn_write.metadata.protocol_version = 763;
        conn_write.state = match self.next_state.get_val() {
            1 => State::Status,
            2 => State::Login,
            s => {return Err(Error::InvalidState(s as u32))}
        };

        Ok(())
    }
}