use crate::errors::NetError;
use crate::utils::broadcast::{broadcast, BroadcastOptions};
use ferrumc_macros::{event_handler, packet, Event, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalState;
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = 0x03)]
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
    //TODO change this global broadcast to a broadcast that affects only players in the view distance
    //      of the player doing it, but as long as we still cant see other players, this will be fine.

    broadcast(
        &event.packet,
        &state,
        BroadcastOptions::default().except([event.eid.val as usize]),
    )
        .await?;
    Ok(event)
}

impl EntityAnimationPacket {
    pub fn new(eid: VarInt, animation: u8) -> Self {
        Self { eid, animation }
    }
}

impl EntityAnimationEvent {
    pub fn new(eid: VarInt, animation: u8) -> Self {
        Self {
            eid: eid.clone(),
            animation: animation,
            packet: EntityAnimationPacket::new(eid, animation.clone()),
        }
    }
}
