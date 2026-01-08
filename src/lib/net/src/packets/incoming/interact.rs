//! Interact Entity packet.
//!
//! Sent when a player interacts with another entity (attack, use, etc).

use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::var_int::VarInt;

/// Interaction types for the interact packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum InteractionType {
    /// Interact with the entity (right-click)
    Interact = 0,
    /// Attack the entity (left-click)
    Attack = 1,
    /// Interact at a specific position
    InteractAt = 2,
}

/// Sent when a player interacts with an entity.
///
/// This packet is used for both attacking (left-click) and interacting (right-click).
#[derive(NetDecode, Debug)]
#[packet(packet_id = "interact", state = "play")]
pub struct InteractEntity {
    /// The entity ID being interacted with
    pub entity_id: VarInt,
    /// The type of interaction
    pub interaction_type: InteractionType,
    // Note: interact_at has additional target_x, target_y, target_z, hand fields
    // For now we'll only handle the attack case properly
    /// Whether the player is sneaking
    pub sneaking: bool,
}

impl InteractEntity {
    /// Check if this is an attack interaction.
    pub fn is_attack(&self) -> bool {
        self.interaction_type == InteractionType::Attack
    }
}
