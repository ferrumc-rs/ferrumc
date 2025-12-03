use crate::codec::net_types::var_int::VarInt;
use ferrumc_macros::{NetDecode, packet};

// Mojang surely has SOME naming schemes.. commands??
#[derive(NetDecode)]
#[packet(id = ids::PLAY_SERVERBOUND_PLAYER_COMMAND, state = "play")]
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
