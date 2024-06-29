use tracing::{info, trace};
use ferrumc_macros::{Decode, packet};
use crate::Connection;
use crate::packets::IncomingPacket;

#[derive(Decode)]
#[packet(packet_id = 0x14, state = "play")]
pub struct SetPlayerPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub on_ground: bool
}

impl IncomingPacket for SetPlayerPosition {
    async fn handle(&self, conn: &mut Connection) -> ferrumc_utils::prelude::Result<()> {
        trace!("SetPlayerPosition packet received");
        trace!("X: {}", self.x);
        trace!("Y: {}", self.y);
        trace!("Z: {}", self.z);
        Ok(())
    }
}