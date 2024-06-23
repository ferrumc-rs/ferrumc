use log::info;
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
    async fn handle(&self, conn: &mut Connection) -> Result<Option<Vec<u8>>, Error> {
        info!("Handling handshake packet");

        conn.state = match self.next_state.get_val() {
            1 => State::Status,
            2 => State::Login,
            s => {return Err(Error::InvalidState(s as u32))}
        };
        Ok(None)
    }
}