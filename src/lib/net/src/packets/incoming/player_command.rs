use crate::packets::IncomingPacket;
use crate::NetResult;
use ferrumc_ecs::entities::Entity;
use ferrumc_events::infrastructure::Event;
use ferrumc_macros::{packet, Event, NetDecode};
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::ServerState;
use std::sync::Arc;

// Mojang surely has SOME naming schemes.. commands??
#[derive(NetDecode)]
#[packet(packet_id = "player_command", state = "play")]
pub struct PlayerCommandPacket {
    entity_id: VarInt,
    // Originally: Action Id = VarInt Enum
    action: PlayerCommandAction,
    jump_boost: VarInt,
}

#[derive(Debug, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.val as u8")]
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

impl IncomingPacket for PlayerCommandPacket {
    async fn handle(self, _: Entity, state: Arc<ServerState>) -> NetResult<()> {
        PlayerDoActionEvent::trigger(PlayerDoActionEvent::from(self), state).await?;
        Ok(())
    }
}

#[derive(Debug, Event)]
pub struct PlayerDoActionEvent {
    pub entity_id: Entity,
    pub action: PlayerCommandAction,
    pub jump_boost: i32,
}

impl From<PlayerCommandPacket> for PlayerDoActionEvent {
    fn from(packet: PlayerCommandPacket) -> Self {
        Self {
            entity_id: packet.entity_id.val as Entity,
            action: packet.action,
            jump_boost: packet.jump_boost.val,
        }
    }
}
