use tracing::trace;

use ferrumc_macros::{packet, Component, NetDecode};

use crate::net::packets::{ConnectionId, IncomingPacket};
use crate::net::systems::chunk_sender::ChunkSender;
use crate::state::GlobalState;

#[derive(NetDecode, Component, Clone, Debug)]
#[packet(packet_id = 0x08, state = "play")]
pub struct ClientInfo {
    pub locale: String,
    pub view_distance: i8,
    pub chat_mode: i8,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: i8,
}

impl IncomingPacket for ClientInfo {
    async fn handle(
        self,
        entity_id: ConnectionId,
        state: GlobalState,
    ) -> crate::utils::prelude::Result<()> {
        trace!("ClientInfo packet received");
        trace!("Locale: {}", self.locale);
        trace!("View Distance: {}", self.view_distance);
        trace!("Chat Mode: {}", self.chat_mode);
        trace!("Chat Colors: {}", self.chat_colors);
        trace!("Displayed Skin Parts: {}", self.displayed_skin_parts);
        trace!("Main Hand: {}", self.main_hand);

        // ClientInfo is a packet & also a component.
        state.world.get_component_storage().insert(entity_id, self);

        // Send chunks again
        ChunkSender::send_chunks_to_player(state.clone(), entity_id).await?;

        Ok(())
    }
}
