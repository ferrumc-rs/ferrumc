use ferrumc_macros::{NetDecode, packet};
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

#[derive(Debug, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum PlayerCommandAction {
    StartSneaking = 0,
    StopSneaking = 1,
    LeaveBed = 2,
    StartSprinting = 3,
    StopSprinting = 4,
    StartJumpWithHorse = 5,
    StopJumpWithHorse = 6,
    OpenVehicleInventory = 7,
    StartFlyingWithElytra = 8,
}
