//! Gives players a test item with components when they join.
//! This is used to verify item component encoding with a real Minecraft client.

use bevy_ecs::prelude::{Added, Query};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_data::items::Item;
use ferrumc_inventories::components::Rarity;
use ferrumc_inventories::{Inventory, ItemBuilder};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::set_container_slot::SetContainerSlot;
use ferrumc_net_codec::net_types::var_int::VarInt;
use tracing::{debug, error, info};

/// Gives new players a test item with components.
/// Uses `Added<PlayerIdentity>` to detect new players after commands are applied.
pub fn handle(
    mut new_players: Query<
        (&PlayerIdentity, &StreamWriter, &mut Inventory),
        Added<PlayerIdentity>,
    >,
) {
    for (identity, writer, mut inventory) in new_players.iter_mut() {

        // Create a test diamond with Epic rarity
        const TEST_SIZE: i32 = 69420;

        let test_item = ItemBuilder::new(Item::DIAMOND.id as i32)
            .max_stack_size(52)
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
                identity.username, err
            );
        } else {
            info!(
                "Gave test item with components to player {}",
                identity.username
            );
            debug!(
                "Test item: Diamond with MaxStackSize(99), Rarity(Epic), EnchantmentGlintOverride(true)"
            );
        }
    }
}
