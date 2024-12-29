use crate::contents::InventoryContents;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::outgoing::open_screen::OpenScreenPacket;
use ferrumc_net::packets::outgoing::set_container_slot::{SetContainerSlotPacket, Slot};
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_text::{TextComponent, TextComponentBuilder};
use std::ops::Deref;
use std::sync::Arc;

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
    Lectern,
    Loom,
    Merchant,
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
                let value = *i as i32;
                if value >= 1 && value <= 6 {
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
            InventoryType::Lectern => 17,
            InventoryType::Loom => 18,
            InventoryType::Merchant => 19,
            InventoryType::ShulkerBox => 20,
            InventoryType::SmithingTable => 21,
            InventoryType::Smoker => 22,
            InventoryType::Cartography => 23,
            InventoryType::Stonecutter => 24,
        };

        VarInt::new(id)
    }
}

pub struct Inventory {
    pub id: VarInt,
    pub inventory_type: InventoryType,
    pub contents: InventoryContents,
    pub title: TextComponent,
}

impl Inventory {
    pub fn new<S: Into<String>>(id: i32, title: S, inventory_type: InventoryType) -> Self {
        Self {
            id: VarInt::new(id),
            inventory_type,
            contents: InventoryContents::empty(),
            title: TextComponentBuilder::new(title).build(),
        }
    }

    pub async fn send_packet(self, writer: &mut StreamWriter) -> Result<(), NetError> {
        let id = self.id;
        let packet = OpenScreenPacket::new(id.clone(), self.inventory_type.get_id(), self.title);

        writer
            .send_packet(&packet, &NetEncodeOpts::WithLength)
            .await?;

        // Temporary until i get container content setup
        for slot in self.contents.contents.iter() {
            let slot_packet = SetContainerSlotPacket::new(
                id.clone(),
                *slot.key() as i16,
                Slot::with_item(1, *slot.value()),
            );
            writer
                .send_packet(&slot_packet, &NetEncodeOpts::SizePrefixed)
                .await?;
        }

        Ok(())
    }
}
