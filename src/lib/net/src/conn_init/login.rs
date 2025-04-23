use crate::errors::NetError;
use ferrumc_state::GlobalState;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

pub(super) async fn login(
    mut conn_read: &mut OwnedReadHalf,
    conn_write: &mut OwnedWriteHalf,
    state: GlobalState,
) -> Result<bool, NetError> {
    Ok(true)
}
