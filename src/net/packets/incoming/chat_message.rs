use crate::net::packets::{ConnectionId, IncomingPacket};
use crate::state::GlobalState;
use crate::utils::components::player::Player;
use ferrumc_macros::{packet, Decode};
use tracing::debug;

#[derive(Decode)]
#[packet(packet_id = 0x05, state = "play")]
pub struct PacketChatMessage {
    pub message: String,
    pub timestamp: i64,
}

impl IncomingPacket for PacketChatMessage {
    async fn handle(
        self,
        conn_id: ConnectionId,
        state: GlobalState,
    ) -> crate::utils::prelude::Result<()> {
        let my_id = conn_id;

        let my_player = state.world.get_component::<Player>(my_id).await?;

        debug!("[{}]: {}", my_player.username, self.message);

        Ok(())
    }
}
