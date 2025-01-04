use ferrumc_inventory::inventory::Inventory;
use ferrumc_inventory::player_inventory::PlayerInventory;
use ferrumc_inventory::slot::Slot;
use ferrumc_macros::event_handler;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::incoming::close_container::InventoryCloseEvent;
use ferrumc_net::packets::incoming::set_creative_mode_slot::SetCreativeModeSlotEvent;
use ferrumc_net::packets::incoming::set_held_item::ChangeSlotEvent;
use ferrumc_state::GlobalState;

#[event_handler]
async fn container_close(
    container_close_event: InventoryCloseEvent,
    state: GlobalState,
) -> Result<InventoryCloseEvent, NetError> {
    let conn_id = container_close_event.conn_id;

    let window_id = container_close_event.window_id;
    if window_id != 0 {
        state.universe.remove_component::<Inventory>(conn_id)?;
    }

    Ok(container_close_event)
}

#[event_handler]
async fn set_creative_mode_slot(
    creative_mode_slot: SetCreativeModeSlotEvent,
    state: GlobalState,
) -> Result<SetCreativeModeSlotEvent, NetError> {
    let conn_id = creative_mode_slot.conn_id;
    let packet = &creative_mode_slot.packet;

    let mut inventory = state.universe.get_mut::<PlayerInventory>(conn_id)?;
    inventory.set_slot(
        packet.slot as i32,
        Slot::from_network_slot(packet.clicked_item),
    );

    Ok(creative_mode_slot)
}

#[event_handler]
async fn handle_carried_item(
    change_slot_event: ChangeSlotEvent,
    state: GlobalState,
) -> Result<ChangeSlotEvent, NetError> {
    let conn_id = change_slot_event.conn_id;

    let mut inventory = state.universe.get_mut::<PlayerInventory>(conn_id)?;
    inventory.set_carried_item(change_slot_event.packet.slot);

    Ok(change_slot_event)
}
