use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::fmt::Display;
use typename::TypeName;

#[derive(TypeName, Debug, NetDecode)]
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
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
}

impl Display for ChatMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatMode::Enabled => write!(f, "Enabled"),
            ChatMode::CommandsOnly => write!(f, "CommandsOnly"),
            ChatMode::Hidden => write!(f, "Hidden"),
        }
    }
}

#[derive(Debug, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum MainHand {
    Left,
    Right,
}

impl Display for MainHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainHand::Left => write!(f, "Left"),
            MainHand::Right => write!(f, "Right"),
        }
    }
}

// impl IncomingPacket for ClientInformation {
//     fn handle(self, conn_id: usize, state: Arc<ServerState>) -> Result<(), NetError> {
//         debug!("Received client information: {:#?}", self);
// 
//         state.universe.add_component(conn_id, self)?;
// 
//         Ok(())
//     }
// }
