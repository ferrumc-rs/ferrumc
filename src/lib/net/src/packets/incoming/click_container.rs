use crate::packets::outgoing::set_container_slot::NetworkSlot;
use crate::packets::IncomingPacket;
use crate::NetResult;
use ferrumc_events::infrastructure::Event;
use ferrumc_macros::{packet, Event, NetDecode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::ServerState;
use std::sync::Arc;
use tracing::info;

#[derive(NetDecode, Debug)]
pub struct ChangedSlots {
    pub slot_number: u16,
    pub slot: NetworkSlot,
}

impl ChangedSlots {
    pub fn new(slot_number: u16, slot: NetworkSlot) -> Self {
        Self { slot_number, slot }
    }
}

#[derive(NetDecode, Debug)]
#[packet(packet_id = 0x0E, state = "play")]
pub struct ClickContainerPacket {
    pub window_id: u8,
    pub state_id: VarInt,
    pub slot: u16,
    pub button: u8,
    pub mode: VarInt,
    pub changed_slots: LengthPrefixedVec<ChangedSlots>,
    pub carried_item: NetworkSlot,
}

impl IncomingPacket for ClickContainerPacket {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        info!("Clicked Container: {:#?}", self);

        let event = InventoryClickEvent::new(conn_id, self);
        InventoryClickEvent::trigger(event, state).await?;
        Ok(())
    }
}

impl ClickContainerPacket {
    pub fn new(
        window_id: u8,
        state_id: VarInt,
        slot: u16,
        button: u8,
        mode: VarInt,
        changed_slots: LengthPrefixedVec<ChangedSlots>,
        carried_item: NetworkSlot,
    ) -> Self {
        Self {
            window_id,
            state_id,
            slot,
            button,
            mode,
            changed_slots,
            carried_item,
        }
    }
}

#[derive(Event, Debug)]
pub struct InventoryClickEvent {
    pub conn_id: usize,
    pub packet: ClickContainerPacket,
    pub is_cancelled: bool,
}

impl InventoryClickEvent {
    pub fn new(conn_id: usize, packet: ClickContainerPacket) -> Self {
        Self {
            conn_id,
            packet,
            is_cancelled: false,
        }
    }

    pub fn set_cancelled(&mut self, is_cancelled: bool) {
        self.is_cancelled = is_cancelled;
    }
}
