use crate::packets::IncomingPacket;
use crate::NetResult;
use ferrumc_events::infrastructure::Event;
use ferrumc_macros::{packet, Event, NetDecode};
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::ServerState;
use std::sync::Arc;

#[derive(NetDecode, Debug)]
#[packet(packet_id = 0x38, state = "play")]
pub struct UseItemOnPacket {
    pub hand: VarInt,
    pub location: NetworkPosition,
    pub face: VarInt,
    pub cursor_pos_x: f32,
    pub cursor_pos_y: f32,
    pub cursor_pos_z: f32,
    pub inside_block: bool,
    pub sequence: VarInt,
}

impl IncomingPacket for UseItemOnPacket {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        let event = UseItemOnEvent::new(self, conn_id);
        UseItemOnEvent::trigger(event, state).await?;
        Ok(())
    }
}

#[derive(Event, Debug)]
pub struct UseItemOnEvent {
    pub conn_id: usize,
    pub packet: UseItemOnPacket,
}

impl UseItemOnEvent {
    pub fn new(packet: UseItemOnPacket, conn_id: usize) -> Self {
        Self { conn_id, packet }
    }
}
