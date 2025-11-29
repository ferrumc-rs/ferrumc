use crate::ids;
use ferrumc_macros::{NetEncode, packet};

#[derive(NetEncode, Copy, Clone)]
#[packet(id = ids::PLAY_CLIENTBOUND_ENTITY_EVENT, state = "play")]
pub struct EntityStatus {
    /// The ID of the entity
    pub entity_id: i32,
    /// The status code to send
    pub status: u8,
}
