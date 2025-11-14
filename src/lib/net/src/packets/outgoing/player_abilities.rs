use ferrumc_core::abilities::player_abilities::PlayerAbilities as PlayerAbilitiesComponent;
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

impl PlayerAbilities {
    pub fn from_abilities(abilities: &PlayerAbilitiesComponent) -> Self {
        let flags = (abilities.invulnerable as u8 * 0x01)
            | (abilities.flying as u8 * 0x02)
            | (abilities.may_fly as u8 * 0x04)
            | (abilities.creative_mode as u8 * 0x08);

        Self {
            flags,
            flying_speed: abilities.flying_speed,
            field_of_view_modifier: abilities.walking_speed,
        }
    }
}
