//! Interact Entity packet.
//!
//! Sent when a player interacts with another entity (attack, use, etc).

use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::var_int::VarInt;

/// Interaction types for the interact packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionType {
    /// Interact with the entity (right-click)
    Interact = 0,
    /// Attack the entity (left-click)
    Attack = 1,
    /// Interact at a specific position
    InteractAt = 2,
}

impl From<i32> for InteractionType {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Interact,
            1 => Self::Attack,
            2 => Self::InteractAt,
            _ => Self::Interact,
        }
    }
}

/// Sent when a player interacts with an entity.
///
/// This packet is used for both attacking (left-click) and interacting (right-click).
#[derive(NetDecode, Debug)]
#[packet(packet_id = "interact", state = "play")]
pub struct InteractEntity {
    /// The entity ID being interacted with
    pub entity_id: VarInt,
    /// The type of interaction (0=interact, 1=attack, 2=interact_at)
    pub interaction_type: VarInt,
    // Note: interact_at has additional target_x, target_y, target_z, hand fields
    // For now we'll only handle the attack case properly
    /// Whether the player is sneaking
    pub sneaking: bool,
}

impl InteractEntity {
    /// Get the interaction type as an enum.
    pub fn get_type(&self) -> InteractionType {
        InteractionType::from(self.interaction_type.0)
    }

    /// Check if this is an attack interaction.
    pub fn is_attack(&self) -> bool {
        self.interaction_type.0 == 1
    }
}
