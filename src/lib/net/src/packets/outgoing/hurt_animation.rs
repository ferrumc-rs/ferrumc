//! Hurt Animation packet.
//!
//! Sent to clients to play the hurt animation on an entity.

use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;

/// Packet sent to play the hurt/damage animation on an entity.
///
/// This makes the entity flash red and plays the hurt sound.
#[derive(NetEncode, Debug)]
#[packet(packet_id = "hurt_animation", state = "play")]
pub struct HurtAnimationPacket {
    /// The entity ID that is being hurt
    pub entity_id: VarInt,
    /// The yaw direction the damage came from (for knockback direction)
    pub yaw: f32,
}

impl HurtAnimationPacket {
    /// Create a new hurt animation packet.
    pub fn new(entity_id: i32, yaw: f32) -> Self {
        Self {
            entity_id: VarInt::new(entity_id),
            yaw,
        }
    }
}
