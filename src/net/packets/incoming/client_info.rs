use tracing::trace;

use ferrumc_macros::{packet, Decode};

use crate::net::packets::IncomingPacket;
use crate::state::GlobalState;
use crate::Connection;

#[derive(Decode)]
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
        _: &mut Connection,
        _state: GlobalState,
    ) -> crate::utils::prelude::Result<()> {
        trace!("ClientInfo packet received");
        trace!("Locale: {}", self.locale);
        trace!("View Distance: {}", self.view_distance);
        trace!("Chat Mode: {}", self.chat_mode);
        trace!("Chat Colors: {}", self.chat_colors);
        trace!("Displayed Skin Parts: {}", self.displayed_skin_parts);
        trace!("Main Hand: {}", self.main_hand);
        Ok(())
    }
}
