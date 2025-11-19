use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;

/// Entity Sound Effect packet (0x6D / 109)
///
/// Plays a sound effect from an entity.
#[derive(NetEncode, Clone)]
#[packet(packet_id = "sound_entity", state = "play")]
pub struct EntitySoundEffectPacket {
    /// The sound effect ID
    pub sound_id: VarInt,
    /// The sound category (e.g., hostile, neutral, player, etc.)
    pub sound_category: VarInt,
    /// The entity emitting the sound
    pub entity_id: VarInt,
    /// Volume (1.0 is 100%, can be higher)
    pub volume: f32,
    /// Pitch (1.0 is normal pitch, can range from 0.5 to 2.0)
    pub pitch: f32,
    /// Random seed for sound variations
    pub seed: i64,
}

impl EntitySoundEffectPacket {
    /// Create a new entity sound effect packet
    pub fn new(sound_id: i32, entity_id: i32, volume: f32, pitch: f32) -> Self {
        Self {
            sound_id: VarInt::new(sound_id),
            sound_category: VarInt::new(5), // 5 = neutral category for entities
            entity_id: VarInt::new(entity_id),
            volume,
            pitch,
            seed: rand::random(),
        }
    }

    /// Create a hurt sound for an entity at normal volume and pitch
    pub fn hurt(sound_id: i32, entity_id: i32) -> Self {
        Self::new(sound_id, entity_id, 1.0, 1.0)
    }
}
