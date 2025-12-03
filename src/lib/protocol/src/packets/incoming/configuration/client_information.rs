use ferrumc_macros::NetDecode;
use ferrumc_macros::packet; // Import the attribute macro
use typename::TypeName;

// Import the types from Core
use ferrumc_core::player::settings::{ChatMode, MainHand, ParticleStatus};

#[derive(TypeName, Debug, NetDecode)]
#[packet(id = ids::CONFIGURATION_SERVERBOUND_CLIENT_INFORMATION, state = "configuration")]
pub struct ClientInformationPacket {
    // (Optional: Rename struct to have 'Packet' suffix for consistency?)
    pub locale: String,
    pub view_distance: i8,
    pub chat_mode: ChatMode,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: MainHand,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
    pub particle_status: ParticleStatus,
}
