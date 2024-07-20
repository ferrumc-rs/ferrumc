use std::time::Instant;
use ferrumc_macros::{Decode, packet};
use ferrumc_utils::components::keep_alive::KeepAlive;
use tracing::info;
use crate::{Connection, GET_WORLD};
use crate::packets::IncomingPacket;

#[derive(Decode)]
#[packet(packet_id = 0x12, status = "play")]
pub struct KeepAlivePacketIn{
    pub keep_alive_id: i64,
}
impl IncomingPacket for KeepAlivePacketIn{
    async fn handle(&self, conn: &mut Connection) -> ferrumc_utils::prelude::Result<()> {
        info!("KeepAlivePacketIn: {:?}", self);
        let player = conn.metadata.entity?;
        info!("Player: {:?}", player);
        let world = GET_WORLD();

        let world = world.read().await;
        let keep_alive = world.get_component_storage().get_mut::<KeepAlive>(&player)?;
        info!("KeepAlive saved data : {:?}", keep_alive);

        let delta = Instant::now() - keep_alive.last_sent;
        info!("It's been {} ticks since the last keep alive packet", delta.as_millis());

        keep_alive.last_received = Instant::now();

        Ok(())
    }
}