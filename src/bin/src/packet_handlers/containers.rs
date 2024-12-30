use tracing::info;
use ferrumc_inventory::inventory::Inventory;
use ferrumc_macros::event_handler;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::incoming::close_container::InventoryCloseEvent;
use ferrumc_state::GlobalState;

#[event_handler]
async fn container_close(
    container_close_event: InventoryCloseEvent,
    state: GlobalState,
) -> Result<InventoryCloseEvent, NetError> {
    let conn_id = container_close_event.conn_id;

    state.universe.remove_component::<Inventory>(conn_id)?;
    Ok(container_close_event)
}