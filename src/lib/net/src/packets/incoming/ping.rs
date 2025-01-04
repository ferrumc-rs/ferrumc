use crate::connection::StreamWriter;
use crate::packets::outgoing::ping_response::PongPacket;
use crate::packets::IncomingPacket;
use crate::NetResult;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_state::ServerState;
use std::sync::Arc;

#[derive(NetDecode, Debug)]
#[packet(packet_id = "ping_request", state = "status")]
pub struct PingPacket {
    payload: i64,
}

impl IncomingPacket for PingPacket {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        let response = PongPacket::new(self.payload);

        let mut writer = state.universe.get_mut::<StreamWriter>(conn_id)?;

        writer
            .send_packet(&response, &NetEncodeOpts::WithLength)
            .await?;

        Ok(())
    }
}
