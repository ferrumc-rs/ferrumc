//! Debug command: Gives the executing player a test item with components.
//!
//! This is used to verify item component encoding with a real Minecraft client.
//! Usage: /givetest

use bevy_ecs::prelude::{Entity, Query};
use ferrumc_commands::Sender;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_data::items::Item;
use ferrumc_inventories::components::Rarity;
use ferrumc_inventories::{Inventory, ItemBuilder};
use ferrumc_macros::command;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::set_container_slot::SetContainerSlot;
use ferrumc_net_codec::net_types::var_int::VarInt;

/// Test item count - set high to verify component encoding visually.
const TEST_COUNT: i32 = 64;

/// Slot index for the test item (slot 38 = third hotbar slot).
const TEST_SLOT: i16 = 38;

#[command("givetest")]
fn givetest_command(
    #[sender] sender: Sender,
    args: (Query<(Entity, &PlayerIdentity, &StreamWriter, &mut Inventory)>,),
) {
    let (mut query,) = args;

    // Find the player who sent the command
    let sender_entity = match &sender {
        Sender::Player(entity) => *entity,
        Sender::Server => {
            sender.send_message("This command can only be used by players.".into(), false);
            return;
        }
    };

    // Find the sender in the query
    let Some((_, identity, writer, mut inventory)) =
        query.iter_mut().find(|(e, _, _, _)| *e == sender_entity)
    else {
        sender.send_message("Failed to find player data.".into(), false);
        return;
    };

    let test_item = ItemBuilder::new(Item::DIAMOND.id as i32)
        .count(TEST_COUNT)
        .custom_name("Epic Diamond")
        .rarity(Rarity::Epic)
        .enchantment_glint(true)
        .lore(["A legendary gem", "Forged in starfire"])
        .build();

    if let Err(err) = inventory.set_item(TEST_SLOT as usize, test_item.clone()) {
        sender.send_message(
            format!("Failed to set test item in inventory: {:?}", err).into(),
            false,
        );
        return;
    }

    let packet = SetContainerSlot {
        window_id: VarInt::new(0),
        state_id: VarInt::new(0),
        slot_index: TEST_SLOT,
        slot: test_item,
    };

    if let Err(err) = writer.send_packet_ref(&packet) {
        sender.send_message(
            format!("Failed to send item packet: {:?}", err).into(),
            false,
        );
        return;
    }

    sender.send_message(
        format!("Gave test diamond to {}", identity.username).into(),
        false,
    );
}
