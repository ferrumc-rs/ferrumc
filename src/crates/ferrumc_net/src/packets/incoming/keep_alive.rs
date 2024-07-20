use std::time::Instant;
use ferrumc_macros::{Decode, packet};
use ferrumc_utils::components::keep_alive::KeepAlive;
use tracing::info;
use crate::{Connection, GET_WORLD};
use crate::packets::IncomingPacket;

#[derive(Decode, Debug)]
#[packet(packet_id = 0x12, state = "play")]
pub struct KeepAlivePacketIn{
    pub keep_alive_id: i64,
}
impl IncomingPacket for KeepAlivePacketIn{
    async fn handle(&self, conn: &mut Connection) -> ferrumc_utils::prelude::Result<()> {
        info!("KeepAlivePacketIn: {:?}", self);

        let player = &conn.metadata.entity;


        info!("Player: {:?}", player);
        let world = GET_WORLD();

        let mut world = world.write().await;
        let Some(keep_alive) = world.get_component_storage_mut().get_mut::<KeepAlive>(player) else {
            return Err(Error::InvalidComponentStorage("KeepAlive component not found".to_string()));
        };
        info!("KeepAlive saved data : {:?}", keep_alive);

        let delta = Instant::now() - keep_alive.last_sent;
        info!("It's been {} ticks since the last keep alive packet", delta.as_millis());

        keep_alive.last_received = Instant::now();

        Ok(())
    }
}