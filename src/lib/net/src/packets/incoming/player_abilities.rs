use ferrumc_macros::{packet, NetDecode};

// The vanilla client sends this packet when the player
// starts/stops flying with the Flags parameter changed accordingly.

// Notes:
// Bit mask. 0x02 is flying.

#[derive(NetDecode)]
#[packet(packet_id = "player_abilities", state = "play")]
pub struct PlayerAbilities {
    pub flags: u8,
}
