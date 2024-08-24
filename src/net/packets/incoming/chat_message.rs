use crate::net::packets::IncomingPacket;
use crate::net::Connection;
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
        conn: &mut Connection,
        state: GlobalState,
    ) -> crate::utils::prelude::Result<()> {
        let my_id = conn.id;

        let my_player = state
            .world
            .get_component::<Player>(my_id)
            .await
            .ok_or(Error::ComponentNotFound("Player".to_string(), my_id as u64))?;

        debug!("[{}]: {}", my_player.username, self.message);

        Ok(())
    }
}
