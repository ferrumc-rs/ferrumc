use ferrumc_ecs::entities::Entity;
use ferrumc_macros::{packet, Event, NetEncode};
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = 0x03)]
pub struct EntityAnimationPacket {
    pub eid: Entity,
    pub animation: u8,
}

#[derive(Event)]
pub struct EntityAnimationEvent {
    pub entity: Entity,
    pub animation: u8,
    pub packet: EntityAnimationPacket,
}

impl EntityAnimationPacket {
    pub fn new(eid: Entity, animation: u8) -> Self {
        Self { eid, animation }
    }
}

impl EntityAnimationEvent {
    pub fn new(eid: Entity, animation: u8) -> Self {
        Self {
            entity: eid,
            animation,
            packet: EntityAnimationPacket::new(eid, animation),
        }
    }
}
