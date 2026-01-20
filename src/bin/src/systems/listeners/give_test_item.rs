//! Gives players a test item with components when they join.
//! This is used to verify item component encoding with a real Minecraft client.

use bevy_ecs::prelude::{Entity, MessageReader, Query};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_data::items::Item;
use ferrumc_inventories::components::{Component, Rarity};
use ferrumc_inventories::slot::InventorySlot;
use ferrumc_inventories::{Inventory, ItemBuilder};
use ferrumc_messages::player_join::PlayerJoined;
use ferrumc_nbt::NBT;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::set_container_slot::SetContainerSlot;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_text::TextComponent;
use tracing::{debug, error, info};

/// Listens for `PlayerJoined` events and gives the player a test item with components.
pub fn handle(
    mut events: MessageReader<PlayerJoined>,
    player_query: Query<(Entity, &PlayerIdentity)>,
    mut inventory_query: Query<(&StreamWriter, &mut Inventory)>,
) {
    for event in events.read() {
        let joined_player = &event.0;

        // Find the entity for the player who joined
        let Some(entity) = player_query
            .iter()
            .find(|(_, identity)| identity.uuid == joined_player.uuid)
            .map(|(entity, _)| entity)
        else {
            error!(
                "Could not find entity for player {} ({})",
                joined_player.username, joined_player.uuid
            );
            continue;
        };

        // Get the player's inventory and stream writer
        let Ok((writer, mut inventory)) = inventory_query.get_mut(entity) else {
            error!(
                "Could not get inventory/writer for player {}",
                joined_player.username
            );
            continue;
        };

        // Create a test diamond with Epic rarity
        // Diamond item
        const TEST_SIZE: i32 = 69420;
        /*let test_item = InventorySlot::with_components(
            ferrumc_data::items::Item::DIAMOND.id as i32, // Diamond
            TEST_SIZE,                                    // Count
            vec![
                Component::Rarity(Rarity::Epic),
                Component::EnchantmentGlintOverride(true),
                Component::MaxStackSize(VarInt::new(TEST_SIZE)),
                Component::CustomName(NBT::new(TextComponent::from("bismillah sigma balsl "))),
            ],
        )*/
        ;

        let test_item = ItemBuilder::new(Item::DIAMOND.id as i32)
            .count(TEST_SIZE)
            .custom_name("Epic Diamond")
            .rarity(Rarity::Epic)
            .enchantment_glint(true)
            .lore(["A legendary gem", "Forged in starfire"])
            .build();

        // Add to first slot (slot 36 = first hotbar slot in player inventory)
        let slot_index: i16 = 38;
        if let Err(err) = inventory.set_item(slot_index as usize, test_item.clone()) {
            error!("Failed to set item in inventory: {:?}", err);
            continue;
        }

        // Send packet to client
        let packet = SetContainerSlot {
            window_id: VarInt::new(0), // Player inventory
            state_id: VarInt::new(0),
            slot_index,
            slot: test_item,
        };

        if let Err(err) = writer.send_packet_ref(&packet) {
            error!(
                "Failed to send test item packet to {}: {:?}",
                joined_player.username, err
            );
        } else {
            info!(
                "Gave test item with components to player {}",
                joined_player.username
            );
            debug!(
                "Test item: Diamond with MaxStackSize(99), Rarity(Epic), EnchantmentGlintOverride(true)"
            );
        }
    }
}
