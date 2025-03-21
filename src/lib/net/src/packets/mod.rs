use crate::NetResult;

pub mod incoming;
pub mod outgoing;
pub mod packet_events;

pub trait IncomingPacket {
    fn handle(
        self,
        conn_id: usize,
        state: std::sync::Arc<ferrumc_state::ServerState>,
    ) -> NetResult<()>;
}
