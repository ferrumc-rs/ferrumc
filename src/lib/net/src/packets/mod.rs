use crate::NetResult;

pub mod incoming;
pub mod outgoing;
pub mod packet_events;

pub trait IncomingPacketBoxed {
    fn handle_boxed(
        self: Box<Self>,
        conn_id: usize,
        state: std::sync::Arc<ferrumc_state::ServerState>,
    ) -> NetResult<()>;
}

pub trait IncomingPacket: IncomingPacketBoxed {
    fn handle(
        self,
        conn_id: usize,
        state: std::sync::Arc<ferrumc_state::ServerState>,
    ) -> NetResult<()>;
}

impl<T: IncomingPacket> IncomingPacketBoxed for T {
    fn handle_boxed(
        self: Box<Self>,
        conn_id: usize,
        state: std::sync::Arc<ferrumc_state::ServerState>,
    ) -> NetResult<()> {
        (*self).handle(conn_id, state)
    }
}

impl<T: IncomingPacket + ?Sized> IncomingPacket for Box<T> {
    fn handle(
        self,
        conn_id: usize,
        state: std::sync::Arc<ferrumc_state::ServerState>,
    ) -> NetResult<()> {
        self.handle_boxed(conn_id, state)
    }
}
