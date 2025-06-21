use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::entity_animation::EntityAnimationPacket;
use ferrumc_net::SwingArmPacketReceiver;
use ferrumc_net_codec::net_types::var_int::VarInt;
use tracing::error;

pub fn handle(
    events: Res<SwingArmPacketReceiver>,
    query: Query<&PlayerIdentity>,
    conn_query: Query<(Entity, &StreamWriter)>,
) {
    for (event, eid) in events.0.try_iter() {
        let animation = {
            if event.hand == 0 {
                0
            } else {
                3
            }
        };
        let game_id = query.get(eid).expect("Game ID not found");
        let packet = EntityAnimationPacket::new(VarInt::new(game_id.short_uuid), animation);
        for (entity, conn) in conn_query.iter() {
            if entity == eid {
                continue; // Skip sending to the player who triggered the event
            }
            if !conn.running.load(std::sync::atomic::Ordering::Relaxed) {
                continue;
            }
            if let Err(e) = conn.send_packet(packet.clone()) {
                error!("Failed to send packet: {}", e);
            }
        }
    }
}
