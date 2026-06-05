use ferrumc_macros::{packet, NetEncode};

/// Clientbound "Set Ticking State" (`minecraft:ticking_state`).
///
/// Tells the client the server's tick rate — which the F3 debug screen displays — and whether ticking
/// is currently frozen. Broadcasting the measured tick rate lets players watch real server TPS while
/// debugging.
#[derive(NetEncode)]
#[packet(packet_id = "ticking_state", state = "play")]
pub struct SetTickingStatePacket {
    /// Ticks per second the server is running at.
    pub tick_rate: f32,
    /// Whether ticking is frozen (as by `/tick freeze`).
    pub is_frozen: bool,
}

impl SetTickingStatePacket {
    pub fn new(tick_rate: f32, is_frozen: bool) -> Self {
        Self {
            tick_rate,
            is_frozen,
        }
    }
}
