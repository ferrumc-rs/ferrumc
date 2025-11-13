use ferrumc_macros::{packet, NetEncode};

#[derive(NetEncode)]
#[packet(packet_id = "player_abilities", state = "play")]
pub struct PlayerAbilities {
    pub flags: u8,                   // Bit field, see below.
    pub flying_speed: f32,           // 0.05 by default.
    pub field_of_view_modifier: f32, // Modifies field of view, like a speed potion.
}

// About flags
// Field            Bit
// Invulnerable:    0x01
// Flying:          0x02
// Allow Flying:    0x04
// Creative Mode:   0x08
