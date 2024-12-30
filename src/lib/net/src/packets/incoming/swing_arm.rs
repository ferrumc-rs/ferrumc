use crate::packets::outgoing::entity_animation::EntityAnimationEvent;
use crate::packets::IncomingPacket;
use crate::NetResult;
use ferrumc_events::infrastructure::Event;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::ServerState;
use std::sync::Arc;
use ferrumc_ecs::entities::Entity;

#[derive(NetDecode)]
#[packet(packet_id = 0x36, state = "play")]
pub struct SwingArmPacket {
    hand: VarInt,
}

impl IncomingPacket for SwingArmPacket {
    async fn handle(self, conn_id: Entity, state: Arc<ServerState>) -> NetResult<()> {
        let animation = {
            if self.hand == 0 {
                0
            } else {
                3
            }
        };
        let event = EntityAnimationEvent::new(conn_id, animation);
        EntityAnimationEvent::trigger(event, state).await?;
        Ok(())
    }
}
