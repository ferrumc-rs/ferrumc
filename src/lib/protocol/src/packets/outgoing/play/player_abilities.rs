use crate::ids;
use ferrumc_components::player::abilities::PlayerAbilities as PlayerAbilitiesComponent;
use ferrumc_macros::{NetEncode, packet};

#[derive(NetEncode)]
#[packet(id = ids::PLAY_CLIENTBOUND_PLAYER_ABILITIES, state = "play")]
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
        let flags = (abilities.invulnerable as u8)
            | (abilities.flying as u8 * 0x02)
            | (abilities.may_fly as u8 * 0x04)
            | (abilities.instant_build as u8 * 0x08);

        Self {
            flags,
            flying_speed: abilities.flying_speed,
            field_of_view_modifier: abilities.walking_speed,
        }
    }
}
