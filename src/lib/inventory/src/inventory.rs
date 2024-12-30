use crate::contents::InventoryContents;
use crate::events::inventory_open::OpenInventoryEvent;
use crate::slot::Slot;
use dashmap::DashMap;
use ferrumc_ecs::entities::Entity;
use ferrumc_ecs::errors::ECSError;
use ferrumc_events::infrastructure::Event;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::incoming::close_container::InventoryCloseEvent;
use ferrumc_net::packets::outgoing::close_container::CloseContainerPacket;
use ferrumc_net::packets::outgoing::open_screen::OpenScreenPacket;
use ferrumc_net::packets::outgoing::set_container_slot::SetContainerSlotPacket;
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::ServerState;
use ferrumc_text::{TextComponent, TextComponentBuilder};
use std::sync::Arc;
use thiserror::Error;

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
    pub fn get_id(&self) -> VarInt {
        let id = match self {
            InventoryType::Chest(i) => {
                let value = i32::from(*i);
                if (1..=6).contains(&value) {
                    value - 1
                } else {
                    0 // defaults to 1 row chest
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
        };

        VarInt::new(id)
    }

    pub fn get_size(&self) -> i32 {
        match self {
            InventoryType::Chest(i) => i32::from(*i) * 9,
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

#[derive(Error, Debug)]
pub enum InventoryError {
    #[error("Entity [{0}] already has an open inventory. Cannot open another one.")]
    AlreadyOpenedInventory(Entity),

    #[error("Net error: [{0}].")]
    NetError(#[from] NetError),

    #[error("ECS error: [{0}].")]
    ECSError(#[from] ECSError),

    #[error("Unknown error occurred with inventories...")]
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Inventory {
    pub id: VarInt,
    pub inventory_type: InventoryType,
    pub title: TextComponent,
    pub(crate) contents: InventoryContents,
}

impl Inventory {
    pub fn new<S: Into<String>>(id: i32, title: S, inventory_type: InventoryType) -> Self {
        Self {
            id: VarInt::new(id),
            inventory_type,
            title: TextComponentBuilder::new(title).build(),
            contents: InventoryContents::empty(),
        }
    }

    pub fn set_slot(&mut self, slot_id: i32, slot: Slot) -> &mut Self {
        let size = self.inventory_type.get_size();
        if (0..=size).contains(&slot_id) {
            self.contents.set_slot(slot_id, slot);
        }

        self
    }

    pub fn get_slot(&self, slot_id: i32) -> Option<Slot> {
        let size = self.inventory_type.get_size();
        if (0..=size).contains(&slot_id) {
            self.contents.get_slot(slot_id)
        } else {
            None
        }
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

        let id = self.id;
        let contents = self.get_contents();
        if !contents.is_empty() {
            for slot in contents.iter() {
                writer
                    .send_packet(
                        &SetContainerSlotPacket::new(
                            id,
                            *slot.key() as i16,
                            slot.value().to_network_slot(),
                        ),
                        &NetEncodeOpts::WithLength,
                    )
                    .await?;
            }
        }

        // handle event
        let event = OpenInventoryEvent::new(entity_id).inventory_id(*id);
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
                &CloseContainerPacket::new(*self.id as u8),
                &NetEncodeOpts::WithLength,
            )
            .await?;

        // handle event
        let event = InventoryCloseEvent::new(entity_id, *inventory.id as u8);
        InventoryCloseEvent::trigger(event, state.clone()).await?;
        Ok(())
    }

    pub fn get_contents(&self) -> &DashMap<i32, Slot> {
        &self.contents.contents
    }

    pub fn clear(&mut self) {
        self.get_contents().clear();
    }

    pub fn set_all(&mut self, slot: Slot) {
        for i in 0..self.get_size() {
            self.set_slot(i, slot);
        }
    }

    pub fn contains(&self, item: i32) -> bool {
        self.get_contents()
            .iter()
            .any(|slot| slot.value().item == item)
    }

    pub fn contains_slot(&self, slot: Slot) -> bool {
        self.contains(slot.item)
    }

    pub fn get_first_empty(&self) -> i32 {
        let contents = self.get_contents();
        for i in 0..self.get_size() {
            if contents.get(&i).is_none() {
                return i;
            }
        }

        0
    }

    pub fn get_size(&self) -> i32 {
        self.inventory_type.get_size()
    }

    pub fn is_empty(&self) -> bool {
        self.get_contents().is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.get_contents().len() == self.get_size() as usize
    }
}
