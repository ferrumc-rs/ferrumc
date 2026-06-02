use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::network_position::NetworkPosition;

/// The clientbound **Level Event** packet.
///
/// Triggers a fixed, client-side audiovisual effect at a position — a sound and/or particles the
/// client already knows how to play, selected by `event_id`. Unlike a raw sound packet this needs
/// no sound-registry id and bundles the matching particles, which is exactly how vanilla plays the
/// lava-extinguish "fizz" when a fluid solidifies.
#[derive(NetEncode)]
#[packet(packet_id = "level_event", state = "play")]
pub struct LevelEventPacket {
    pub event_id: i32,
    pub location: NetworkPosition,
    /// Event-specific extra data; 0 for the effects we use.
    pub data: i32,
    /// When true, the client ignores distance when scaling volume. Always false for us.
    pub disable_relative_volume: bool,
}

impl LevelEventPacket {
    /// Event id for the lava-extinguish effect (`LAVA_FIZZ` / `1501` in vanilla): the hissing
    /// sound plus a puff of smoke, played when lava solidifies on contact with water.
    pub const LAVA_EXTINGUISH: i32 = 1501;

    /// Builds the lava-fizz effect at `location`.
    pub fn lava_extinguish(location: NetworkPosition) -> Self {
        Self {
            event_id: Self::LAVA_EXTINGUISH,
            location,
            data: 0,
            disable_relative_volume: false,
        }
    }
}
