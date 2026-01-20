//! Debug system: Gives players a test item with components when they join.
//!
//! This is used to verify item component encoding with a real Minecraft client.
//! Only compiled in debug builds (behind `#[cfg(debug_assertions)]`).

use bevy_ecs::prelude::{Added, Query};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_data::items::Item;
use ferrumc_inventories::components::Rarity;
use ferrumc_inventories::{Inventory, ItemBuilder};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::set_container_slot::SetContainerSlot;
use ferrumc_net_codec::net_types::var_int::VarInt;
use tracing::{error, info};

/// Test item count - set high to verify component encoding visually.
const TEST_COUNT: i32 = 64;

/// Slot index for the test item (slot 38 = third hotbar slot).
const TEST_SLOT: i16 = 38;

/// Gives new players a test item with components.
/// Uses `Added<PlayerIdentity>` to detect new players after commands are applied.
pub fn handle(
    mut new_players: Query<
        (&PlayerIdentity, &StreamWriter, &mut Inventory),
        Added<PlayerIdentity>,
    >,
) {
    for (identity, writer, mut inventory) in new_players.iter_mut() {
        let test_item = ItemBuilder::new(Item::DIAMOND.id as i32)
            .count(TEST_COUNT)
            .custom_name("Epic Diamond")
            .rarity(Rarity::Epic)
            .enchantment_glint(true)
            .lore(["A legendary gem", "Forged in starfire"])
            .build();

        if let Err(err) = inventory.set_item(TEST_SLOT as usize, test_item.clone()) {
            error!("Failed to set test item in inventory: {:?}", err);
            continue;
        }

        let packet = SetContainerSlot {
            window_id: VarInt::new(0),
            state_id: VarInt::new(0),
            slot_index: TEST_SLOT,
            slot: test_item,
        };

        if let Err(err) = writer.send_packet_ref(&packet) {
            error!(
                "Failed to send test item packet to {}: {:?}",
                identity.username, err
            );
        } else {
            info!(
                "[DEBUG] Gave test diamond to player {}",
                identity.username
            );
        }
    }
}
