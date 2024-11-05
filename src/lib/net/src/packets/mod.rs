use crate::NetResult;

pub mod incoming;
pub mod outgoing;

#[allow(async_fn_in_trait)]
pub trait IncomingPacket {
    async fn handle(self, conn_id: usize, state: std::sync::Arc<ferrumc_state::ServerState>) -> NetResult<()>;
}