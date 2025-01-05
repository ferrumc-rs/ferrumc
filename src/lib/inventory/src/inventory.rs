use crate::contents::InventoryContents;
use crate::errors::InventoryError;
use crate::slot::Slot;
use std::collections::BTreeMap;

use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_ecs::{components::storage::ComponentRefMut, entities::Entity};
use ferrumc_events::infrastructure::Event;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::incoming::close_container::InventoryCloseEvent;
use ferrumc_net::packets::outgoing::close_container::CloseContainerPacket;
use ferrumc_net::packets::outgoing::open_screen::{OpenInventoryEvent, OpenScreenPacket};
use ferrumc_net::packets::outgoing::set_container_content::SetContainerContentPacket;
use ferrumc_net::packets::outgoing::set_container_slot::SetContainerSlotPacket;
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::ServerState;
use ferrumc_text::TextComponent;
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub enum InventoryType {
    Chest(i8),
    Anvil,
    Beacon,
    BlastFurnace,
    BrewingStand,
    CraftingTable,
    EnchantmentTable,
    Furnace,
    Grindstone,
    Hopper,
    Dispenser,
    Dropper,
    Lectern,
    Loom,
    ShulkerBox,
    SmithingTable,
    Smoker,
    Cartography,
    Stonecutter,
}

impl InventoryType {
    pub fn get_id(&self) -> i32 {
        match self {
            InventoryType::Chest(i) => {
                let value = i32::from(*i);
                if (1..=6).contains(&value) {
                    value - 1
                } else {
                    1 // defaults to 1 row chest
                }
            }
            InventoryType::Anvil => 8,
            InventoryType::Beacon => 9,
            InventoryType::BlastFurnace => 10,
            InventoryType::BrewingStand => 11,
            InventoryType::CraftingTable => 12,
            InventoryType::EnchantmentTable => 13,
            InventoryType::Furnace => 14,
            InventoryType::Grindstone => 15,
            InventoryType::Hopper => 16,
            InventoryType::Dispenser | InventoryType::Dropper => 6,
            InventoryType::Lectern => 17,
            InventoryType::Loom => 18,
            InventoryType::ShulkerBox => 20,
            InventoryType::SmithingTable => 21,
            InventoryType::Smoker => 22,
            InventoryType::Cartography => 23,
            InventoryType::Stonecutter => 24,
        }
    }

    pub fn get_size(&self) -> i16 {
        match self {
            InventoryType::Chest(i) => i16::from(*i) * 9,
            InventoryType::Anvil
            | InventoryType::BlastFurnace
            | InventoryType::Furnace
            | InventoryType::Smoker
            | InventoryType::Cartography
            | InventoryType::Grindstone => 2,
            InventoryType::Stonecutter | InventoryType::EnchantmentTable => 1,
            InventoryType::Dispenser | InventoryType::Dropper => 8,
            InventoryType::Loom | InventoryType::SmithingTable => 3,
            InventoryType::Beacon => 0,
            InventoryType::Hopper => 4,
            InventoryType::ShulkerBox => 26,
            InventoryType::CraftingTable => 9,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum InventorySyncType {
    All,
    Single(i16),
}

#[derive(Debug, Clone)]
pub struct Inventory {
    pub id: u8,
    pub inventory_type: InventoryType,
    pub title: TextComponent,
    pub(crate) contents: InventoryContents,
    pub carried_item: Slot,
    pub is_synced: bool,
}

impl Inventory {
    pub fn set_carried_item(&mut self, carried_item: u16) {
        if !(0..=9).contains(&carried_item) {
            return;
        }

        let slot = self
            .get_slot(36 + (carried_item as i16))
            .unwrap_or_else(Slot::empty);

        self.carried_item = slot;
    }

    pub(crate) async fn send_inventory_slot_content(
        &self,
        slot_num: i16,
        mut writer: ComponentRefMut<'_, StreamWriter>,
    ) -> Result<(), InventoryError> {
        let Some(slot) = self.get_slot(slot_num) else {
            return Err(InventoryError::InvalidSlot);
        };

        writer
            .send_packet(
                &SetContainerSlotPacket::new(
                    VarInt::new(i32::from(self.id)),
                    slot_num,
                    slot.to_network_slot(),
                ),
                &NetEncodeOpts::WithLength,
            )
            .await?;

        Ok(())
    }

    pub(crate) async fn send_inventory_content(
        &self,
        mut writer: ComponentRefMut<'_, StreamWriter>,
    ) -> Result<(), InventoryError> {
        writer
            .send_packet(
                &SetContainerContentPacket::new(
                    self.id,
                    self.contents.construct_packet_contents(),
                    self.carried_item.to_network_slot(),
                ),
                &NetEncodeOpts::WithLength,
            )
            .await?;

        Ok(())
    }

    pub async fn sync_inventory_with(
        &mut self,
        sync_type: &InventorySyncType,
        writer: ComponentRefMut<'_, StreamWriter>,
    ) -> Result<(), InventoryError> {
        match sync_type {
            InventorySyncType::All => self.send_inventory_content(writer).await?,
            InventorySyncType::Single(slot_num) => {
                self.send_inventory_slot_content(*slot_num, writer).await?
            }
        }
        Ok(())
    }

    pub async fn sync_inventory(
        &mut self,
        without_entity: Entity,
        sync_type: &InventorySyncType,
        state: Arc<ServerState>,
    ) -> Result<(), InventoryError> {
        if !self.is_synced {
            return Err(InventoryError::SyncingANonSyncedInventory(self.id));
        }

        let universe = &state.universe;
        let query = universe
            .get_component_manager()
            .get_entities_with::<ChunkReceiver>();

        for entity_id in query {
            if entity_id == without_entity {
                continue;
            }

            let inventory_result = universe.get_mut::<Inventory>(entity_id);
            if let Ok(inventory) = inventory_result {
                if self.id != inventory.id {
                    continue;
                }

                let writer = universe.get_mut::<StreamWriter>(entity_id)?;
                self.sync_inventory_with(sync_type, writer).await?;
            }
        }

        Ok(())
    }

    pub async fn add_viewer(
        self,
        state: Arc<ServerState>,
        entity_id: Entity,
    ) -> Result<(), InventoryError> {
        let universe = &state.universe;
        let mut writer = universe.get_mut::<StreamWriter>(entity_id)?;

        if universe.get::<Inventory>(entity_id).is_ok() {
            return Err(InventoryError::AlreadyOpenedInventory(entity_id));
        }

        let packet =
            OpenScreenPacket::new(self.id, self.inventory_type.get_id(), self.title.clone());

        writer
            .send_packet(&packet, &NetEncodeOpts::WithLength)
            .await?;

        self.send_inventory_content(writer).await?;

        // handle event
        let event = OpenInventoryEvent::new(entity_id, self.id);
        OpenInventoryEvent::trigger(event, state.clone()).await?;

        universe.add_component::<Inventory>(entity_id, self)?;
        Ok(())
    }

    pub async fn remove_viewer(
        &mut self,
        state: Arc<ServerState>,
        entity_id: Entity,
    ) -> Result<(), InventoryError> {
        let universe = &state.universe;
        let mut writer = universe.get_mut::<StreamWriter>(entity_id)?;
        let inventory = universe.get::<Inventory>(entity_id)?;

        writer
            .send_packet(
                &CloseContainerPacket::new(self.id),
                &NetEncodeOpts::WithLength,
            )
            .await?;

        // handle event
        let event = InventoryCloseEvent::new(entity_id, inventory.id);
        InventoryCloseEvent::trigger(event, state.clone()).await?;
        Ok(())
    }

    pub fn set_slot(&mut self, slot_id: i16, slot: Slot) -> &mut Self {
        let size = self.inventory_type.get_size();
        if (0..=size).contains(&slot_id) {
            self.contents.set_slot(slot_id, slot);
        }

        self
    }

    pub fn set_slots(&mut self, slots: Vec<(i16, Slot)>) -> &mut Self {
        for (slot_num, slot) in slots {
            self.set_slot(slot_num, slot);
        }

        self
    }

    pub fn get_slot(&self, slot_id: i16) -> Option<Slot> {
        let size = self.inventory_type.get_size();
        if (0..=size).contains(&slot_id) {
            self.contents.get_slot(slot_id)
        } else {
            None
        }
    }

    pub fn get_contents(&self) -> &BTreeMap<i16, Slot> {
        &self.contents.contents
    }

    pub fn get_contents_mut(&mut self) -> &mut BTreeMap<i16, Slot> {
        &mut self.contents.contents
    }

    pub fn clear(&mut self) {
        self.get_contents_mut().clear();
    }

    pub fn fill(&mut self, slot: Slot) {
        for i in 0..self.get_size() {
            self.set_slot(i, slot);
        }
    }

    pub fn contains(&self, item: i32) -> bool {
        self.get_contents().iter().any(|slot| slot.1.item == item)
    }

    pub fn contains_atleast(&self, item: i32, amount: i32) -> bool {
        let mut container_amount = 0;
        self.get_contents().iter().for_each(|(_, slot)| {
            if slot.item == item {
                container_amount += slot.count;
            }
        });

        container_amount >= amount
    }

    pub fn get_first_empty(&self) -> i16 {
        let contents = self.get_contents();
        for i in 0..self.get_size() {
            if contents.get(&i).is_none() {
                return i;
            }
        }

        0
    }

    pub fn get_size(&self) -> i16 {
        self.inventory_type.get_size()
    }

    pub fn is_empty(&self) -> bool {
        self.get_contents().is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.get_contents().len() == self.get_size() as usize
    }
}
