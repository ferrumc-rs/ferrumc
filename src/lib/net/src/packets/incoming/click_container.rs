use crate::packets::outgoing::set_container_slot::NetworkSlot;
use crate::packets::IncomingPacket;
use crate::NetResult;
use ferrumc_events::infrastructure::Event;
use ferrumc_macros::{packet, Event, NetDecode};
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts, NetDecodeResult};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::ServerState;
use std::io::Read;
use std::sync::Arc;
use tracing::debug;

#[derive(Debug, Clone, Copy)]
pub enum InventoryClickActions {
    // Mode 0 actions
    LeftMouseClick,
    RightMouseClick,
    LeftClickOutsideInventory,
    RightClickOutsideInventory,

    // Mode 1 actions
    ShiftLeftMouseClick,
    ShiftRightMouseClick,

    // Mode 2 actions
    NumberKey(u8), // keys 1 through 9.

    // Mode 3 actions
    OffhandSwap,

    // Mode 4 actions
    MiddleClick,
    DropKey,
    ControlDropKey,

    // Mode 5 actions
    StartMouseDrag { button: u8 }, // Starting drag with a specific button
    AddSlotToMouseDrag { button: u8 }, // Adding slot with a specific button
    EndMouseDrag { button: u8 },   // Ending drag with a specific button

    // Mode 6 actions
    DoubleClick,
    ReversePickup,
}

impl InventoryClickActions {
    pub fn get_action(mode: i32, button: u8, slot: i16) -> Option<Self> {
        match mode {
            0 => match (button, slot) {
                (0, -999) => Some(InventoryClickActions::LeftClickOutsideInventory),
                (_, -999) => Some(InventoryClickActions::RightClickOutsideInventory),
                (0, _) => Some(InventoryClickActions::LeftMouseClick),
                _ => Some(InventoryClickActions::RightMouseClick),
            },
            1 => match button {
                0 => Some(InventoryClickActions::ShiftLeftMouseClick),
                _ => Some(InventoryClickActions::ShiftRightMouseClick),
            },
            2 => match button {
                1..=9 => Some(InventoryClickActions::NumberKey(button)),
                _ => None,
            },
            3 => match button {
                40 => Some(InventoryClickActions::OffhandSwap),
                _ => None,
            },
            4 => match button {
                2 => Some(InventoryClickActions::MiddleClick),
                0 => Some(InventoryClickActions::DropKey),
                1 => Some(InventoryClickActions::ControlDropKey),
                _ => None,
            },
            5 => match (slot, button) {
                (-999, 0) => Some(InventoryClickActions::StartMouseDrag { button }),
                (_, 0) => Some(InventoryClickActions::AddSlotToMouseDrag { button }),
                (_, 1) => Some(InventoryClickActions::AddSlotToMouseDrag { button }),
                (-999, _) => Some(InventoryClickActions::EndMouseDrag { button }),
                _ => None,
            },
            6 => match button {
                0 => Some(InventoryClickActions::DoubleClick),
                1 => Some(InventoryClickActions::ReversePickup),
                _ => None,
            },
            _ => None,
        }
    }
}

#[derive(NetDecode, Debug)]
pub struct ChangedSlots {
    pub slot_number: i16,
    pub slot: NetworkSlot,
}

impl ChangedSlots {
    pub fn new(slot_number: i16, slot: NetworkSlot) -> Self {
        Self { slot_number, slot }
    }
}

#[derive(Debug)]
#[packet(packet_id = 0x0E, state = "play")]
pub struct ClickContainerPacket {
    pub window_id: u8,
    pub state_id: VarInt,
    pub slot: i16,
    pub button: Option<InventoryClickActions>,
    pub mode: VarInt,
    pub changed_slots: LengthPrefixedVec<ChangedSlots>,
    pub carried_item: NetworkSlot,
}

impl NetDecode for ClickContainerPacket {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> NetDecodeResult<Self> {
        let window_id = u8::decode(reader, opts)?;
        let state_id = VarInt::decode(reader, opts)?;
        let slot = i16::decode(reader, opts)?;
        let button = u8::decode(reader, opts)?;
        let mode = VarInt::decode(reader, opts)?;
        let changed_slots = LengthPrefixedVec::<ChangedSlots>::decode(reader, opts)?;
        let carried_item = NetworkSlot::decode(reader, opts)?;

        Ok(Self::new(
            window_id,
            state_id,
            slot,
            InventoryClickActions::get_action(*mode, button, slot),
            mode,
            changed_slots,
            carried_item,
        ))
    }
}

impl IncomingPacket for ClickContainerPacket {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        debug!("{:#?}", self);

        let event = InventoryClickEvent::new(conn_id, self);
        InventoryClickEvent::trigger(event, state).await?;
        Ok(())
    }
}

impl ClickContainerPacket {
    pub fn new(
        window_id: u8,
        state_id: VarInt,
        slot: i16,
        button: Option<InventoryClickActions>,
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
    pub is_canceled: bool,
}

impl InventoryClickEvent {
    pub fn new(conn_id: usize, packet: ClickContainerPacket) -> Self {
        Self {
            conn_id,
            packet,
            is_canceled: false,
        }
    }

    pub fn set_canceled(&mut self, is_cancelled: bool) {
        self.is_canceled = is_cancelled;
    }
}
