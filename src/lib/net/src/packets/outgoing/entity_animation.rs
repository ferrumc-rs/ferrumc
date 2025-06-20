use bevy_ecs::prelude::{Entity, Event};
use ferrumc_macros::{NetEncode, packet};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;

#[derive(NetEncode, Clone)]
#[packet(packet_id = "animate", state = "play")]
pub struct EntityAnimationPacket {
    pub eid: VarInt,
    pub animation: u8,
}

#[derive(Event)]
pub struct EntityAnimationEvent {
    pub entity: Entity,
    pub animation: u8,
    pub packet: EntityAnimationPacket,
}

impl EntityAnimationPacket {
    pub fn new(eid: VarInt, animation: u8) -> Self {
        Self { eid, animation }
    }
}

impl EntityAnimationEvent {
    pub fn new(eid: Entity, animation: u8, game_id: VarInt) -> Self {
        Self {
            entity: eid,
            animation,
            packet: EntityAnimationPacket::new(game_id, animation),
        }
    }
}
