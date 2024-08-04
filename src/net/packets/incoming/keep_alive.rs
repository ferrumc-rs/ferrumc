use std::time::Instant;

use tracing::info;

use ferrumc_macros::{Decode, packet};

use crate::{Connection, GET_WORLD};
use crate::net::packets::IncomingPacket;
use crate::utils::components::keep_alive::KeepAlive;

#[derive(Decode, Debug)]
#[packet(packet_id = 0x12, state = "play")]
pub struct KeepAlivePacketIn {
    pub keep_alive_id: i64,
}
impl IncomingPacket for KeepAlivePacketIn {
    async fn handle(&self, conn: &mut Connection) -> crate::utils::prelude::Result<()> {
        info!("KeepAlivePacketIn: {:?}", self);

        let player = &conn.metadata.entity;


        info!("Player: {:?}", player);
        /*let world = GET_WORLD();

        let mut world = world.write().await;
        let Some(keep_alive) = world.get_component_storage_mut().get_mut::<KeepAlive>(player) else {
            return Err(Error::InvalidComponentStorage("KeepAlive component not found".to_string()));
        };
        info!("KeepAlive saved data : {:?}", keep_alive);

        let delta = Instant::now() - keep_alive.last_sent;
        info!("It's been {:?} since the last keep alive packet", delta);

        keep_alive.last_received = Instant::now();*/

        Ok(())
    }
}