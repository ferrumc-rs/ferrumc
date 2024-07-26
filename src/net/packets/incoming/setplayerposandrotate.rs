use tracing::trace;
use ferrumc_macros::{Decode, packet};

use crate::Connection;
use crate::net::packets::IncomingPacket;

#[derive(Decode)]
#[packet(packet_id = 0x15, state = "play")]
pub struct SetPlayerPosAndRotate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

impl IncomingPacket for SetPlayerPosAndRotate {
    async fn handle(&self, _: &mut Connection) -> crate::utils::prelude::Result<()> {
        trace!("SetPlayerPosAndRotate packet received");
        trace!("X: {}", self.x);
        trace!("Y: {}", self.y);
        trace!("Z: {}", self.z);
        trace!("Yaw: {}", self.yaw);
        trace!("Pitch: {}", self.pitch);
        Ok(())
    }
}
