use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::var_int::VarInt;

/// Packet sent when a player interacts with an entity
///
/// Protocol structure (Minecraft 1.21):
/// - entity_id: VarInt - The entity being interacted with
/// - interaction_type: VarInt
///   - 0 = Interact (right-click)
///   - 1 = Attack (left-click)
///   - 2 = Interact At (right-click at specific location)
///
/// For type 1 (Attack), the only additional field is:
/// - sneaking: Boolean
///
/// Note: Types 0 and 2 have additional fields (hand, coordinates) that would need
/// custom decoding. For now, we primarily handle Attack interactions.
#[derive(NetDecode)]
#[packet(packet_id = "interact", state = "play")]
pub struct InteractEntityPacket {
    pub entity_id: VarInt,
    pub interaction_type: VarInt,
    pub sneaking: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionType {
    Interact = 0,
    Attack = 1,
    InteractAt = 2,
}

impl InteractEntityPacket {
    pub fn get_interaction_type(&self) -> Option<InteractionType> {
        match self.interaction_type.0 {
            0 => Some(InteractionType::Interact),
            1 => Some(InteractionType::Attack),
            2 => Some(InteractionType::InteractAt),
            _ => None,
        }
    }

    pub fn is_attack(&self) -> bool {
        self.interaction_type.0 == 1
    }
}
