use bevy_ecs::prelude::{Query, Res};
use ferrumc_inventories::defined_slots::player::HEAD_SLOT;
use ferrumc_inventories::INVENTORY_UPDATES_QUEUE;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use tracing::{debug, error};

pub fn handle_inventory_updates(state: Res<GlobalStateResource>, mut query: Query<&StreamWriter>) {
    while let Some(update) = INVENTORY_UPDATES_QUEUE.pop() {
        if state.0.players.is_connected(update.entity) {
            if let Ok(writer) = query.get_mut(update.entity) {
                if !writer.running.load(std::sync::atomic::Ordering::Relaxed) {
                    continue;
                }
                let packet = ferrumc_net::packets::outgoing::set_container_slot::SetContainerSlot {
                    window_id: VarInt::new(0),
                    state_id: VarInt::new(0),
                    slot_index: HEAD_SLOT as i16,
                    slot: ferrumc_inventories::slot::InventorySlot {
                        count: VarInt::new(65),
                        item_id: Some(VarInt::new(872)),
                        components_to_add_count: Some(VarInt::new(0)),
                        components_to_remove_count: Some(VarInt::new(0)),
                        components_to_add: None,
                        components_to_remove: None,
                    },
                };
                if let Err(err) = writer.send_packet_ref(&packet) {
                    error!("Failed to send inventory update packet: {:?}", err);
                } else {
                    debug!("Sent inventory update for player {}", update.entity);
                }
            } else {
                error!("Could not find writer for player {}", update.entity);
            }
        } else {
            error!("Player {} is not connected", update.entity);
        }
    }
}
