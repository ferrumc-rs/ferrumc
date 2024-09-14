use tracing::debug;

use ferrumc_macros::{packet, NetDecode};

use crate::net::packets::{ConnectionId, IncomingPacket};
use crate::state::GlobalState;
use crate::utils::components::keep_alive::KeepAlive;

#[derive(NetDecode, Debug)]
#[packet(packet_id = 0x12, state = "play")]
pub struct KeepAlivePacketIn {
    pub keep_alive_id: i64,
}

impl IncomingPacket for KeepAlivePacketIn {
    async fn handle(
        self,
        conn: ConnectionId,
        state: GlobalState,
    ) -> crate::utils::prelude::Result<()> {
        debug!("KeepAlivePacketIn: {:?}", self);

        let player = conn;

        debug!("Player: {:?}", player);

        let mut keep_alive = state.world.get_component_mut::<KeepAlive>(player).await?;

        debug!("KeepAlive for player: {:?}", *keep_alive);

        keep_alive.last_received = std::time::Instant::now();

        Ok(())
    }
}
