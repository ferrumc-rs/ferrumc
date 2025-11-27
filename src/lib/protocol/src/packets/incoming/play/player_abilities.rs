use ferrumc_macros::{packet, NetDecode};
use ferrumc_protocol::ids;

// The vanilla client sends this packet when the player
// starts/stops flying with the Flags parameter changed accordingly.

// Notes:
// Bit mask. 0x02 is flying.

#[derive(NetDecode)]
#[packet(id = ids::PLAY_SERVERBOUND_PLAYER_ABILITIES, state = "play")]
pub struct PlayerAbilities {
    pub flags: u8,
}
