use crate::codec::net_types::var_int::VarInt;
use crate::ids;
use bevy_ecs::prelude::{Entity, Message};
use ferrumc_macros::{NetEncode, packet};

#[derive(NetEncode, Clone)]
#[packet(id = ids::PLAY_CLIENTBOUND_ANIMATE, state = "play")]
pub struct EntityAnimationPacket {
    pub eid: VarInt,
    pub animation: u8,
}

#[derive(Message)]
pub struct EntityAnimated {
    pub entity: Entity,
    pub animation: u8,
    pub packet: EntityAnimationPacket,
}

impl EntityAnimationPacket {
    pub fn new(eid: VarInt, animation: u8) -> Self {
        Self { eid, animation }
    }
}

impl EntityAnimated {
    pub fn new(eid: Entity, animation: u8, game_id: VarInt) -> Self {
        Self {
            entity: eid,
            animation,
            packet: EntityAnimationPacket::new(game_id, animation),
        }
    }
}
