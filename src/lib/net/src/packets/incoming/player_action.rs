use crate::packets::IncomingPacket;
use crate::NetResult;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::ServerState;
use std::sync::Arc;
use tracing::debug;

#[derive(NetDecode)]
#[packet(packet_id = "player_action", state = "play")]
pub struct PlayerAction {
    pub status: VarInt,
    pub location: NetworkPosition,
    pub face: u8,
    pub sequence: VarInt,
}

impl IncomingPacket for PlayerAction {
    async fn handle(self, _: usize, state: Arc<ServerState>) -> NetResult<()> {
        // https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Protocol?oldid=2773393#Player_Action
        match self.status.val {
            0 => {
                state
                    .world
                    .set_block(
                        self.location.x,
                        self.location.y as i32,
                        self.location.z,
                        "overworld",
                        ferrumc_world::vanilla_chunk_format::BlockData {
                            name: "minecraft:air".to_string(),
                            properties: None,
                        },
                    )
                    .await?;
            }
            1 => {
                debug!("You shouldn't be seeing this in creative mode.");
            }
            _ => {}
        };
        Ok(())
    }
}
