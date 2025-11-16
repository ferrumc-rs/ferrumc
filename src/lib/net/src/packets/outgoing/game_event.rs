use ferrumc_macros::{packet, NetEncode};

#[derive(NetEncode)]
#[packet(packet_id = "game_event", state = "play")]
pub struct GameEventPacket {
    pub event_id: u8,
    pub value: f32,
}

impl GameEventPacket {
    /// GameEvent packet ID to indicate no respawn block is available.  
    /// This displays this message to the user:
    /// > "You have no home bed or charged respawn anchor, or it was obstructed"
    pub const NO_RESPAWN_BLOCK_AVAILABLE: u8 = 0;

    /// GameEvent packet ID to begin raining.
    /// The rest of the rain generation is handled client-side.  
    /// To stop the rain, use [`END_RAINING`].
    pub const BEGIN_RAINING: u8 = 1;

    /// GameEvent packet ID to end raining.
    /// The rest of the rain generation is handled client-side.  
    /// To start the rain, use [`BEGIN_RAINING`].
    pub const END_RAINING: u8 = 2;

    /// GameEvent packet ID to change the gamemode of a player.  
    /// Value options:
    /// - 0: Survival
    /// - 1: Creative
    /// - 2: Adventure
    /// - 3: Spectator
    pub const CHANGE_GAME_MODE: u8 = 3;

    pub fn new(event_id: u8, value: f32) -> Self {
        Self { event_id, value }
    }

    pub fn start_waiting_for_level_chunks() -> Self {
        Self::new(13, 0f32)
    }
}
