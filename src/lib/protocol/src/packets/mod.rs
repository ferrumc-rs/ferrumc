use crate::errors::NetError;

pub mod incoming;
pub mod outgoing;

// #[enum_delegate::register]
pub trait IncomingPacket {
    fn handle(
        self,
        conn_id: usize,
        state: std::sync::Arc<ferrumc_components::state::server_state::GlobalState>,
    ) -> Result<(), NetError>;
}
