use crate::packets::IncomingPacket;
use crate::NetResult;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::ServerState;
use std::sync::Arc;
use tracing::debug;

#[derive(Debug, NetDecode)]
#[packet(packet_id = "client_information", state = "configuration")]
pub struct ClientInformation {
    pub locale: String,
    pub view_distance: u8,
    pub chat_mode: ChatMode,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: MainHand,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
}

#[derive(Debug, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.val as u8")]
#[repr(u8)]
pub enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
}

#[derive(Debug, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.val as u8")]
#[repr(u8)]
pub enum MainHand {
    Left,
    Right,
}

impl IncomingPacket for ClientInformation {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        debug!("Received client information: {:#?}", self);

        state.universe.add_component(conn_id, self).await?;

        Ok(())
    }
}
