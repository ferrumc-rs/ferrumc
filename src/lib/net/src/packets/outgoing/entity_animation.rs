
use ferrumc_macros::{event_handler, packet, Event, NetEncode};use std::io::Write;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalState;
use crate::errors::NetError;
use crate::utils::broadcast::{broadcast, BroadcastOptions};

#[derive(NetEncode)]
#[packet(packet_id = 0x03)]
#[derive(Debug)]
pub struct EntityAnimationPacket {
    pub eid: VarInt,
    pub animation: u8,
}

#[derive(Event)]
pub struct EntityAnimationEvent {
    pub eid: VarInt,
    pub animation: u8,
    pub packet: EntityAnimationPacket,
}

#[event_handler]
async fn entity_animation(
    event: EntityAnimationEvent,
    state: GlobalState,
) -> Result<EntityAnimationEvent, NetError> {
    broadcast(&event.packet,&state, BroadcastOptions::default().except([event.eid.val as usize])).await?;
    Ok(event)
}

impl EntityAnimationPacket {
    pub fn new(eid: VarInt, animation: u8) -> Self {
        Self{eid, animation}
    }
}

impl EntityAnimationEvent {
    pub fn new(eid: VarInt, animation: u8) -> Self {
        Self{ eid: eid.clone(),
              animation: animation,
              packet: EntityAnimationPacket::new(eid, animation.clone())
        }
    }
}
