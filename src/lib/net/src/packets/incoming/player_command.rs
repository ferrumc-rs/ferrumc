use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::var_int::VarInt;

// Mojang surely has SOME naming schemes.. commands??
#[derive(NetDecode)]
#[packet(packet_id = "player_command", state = "play")]
pub struct PlayerCommandPacket {
    pub entity_id: VarInt,
    // Originally: Action Id = VarInt Enum
    pub action: PlayerCommandAction,
    pub jump_boost: VarInt,
}

/// Action IDs for PlayerCommand packet (1.21.x protocol)
/// Note: Sneaking is NOT handled here - it uses PlayerInput packet instead
#[derive(Debug, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum PlayerCommandAction {
    LeaveBed = 0,
    StartSprinting = 1,
    StopSprinting = 2,
    StartJumpWithHorse = 3,
    StopJumpWithHorse = 4,
    OpenVehicleInventory = 5,
    StartFlyingWithElytra = 6,
}
