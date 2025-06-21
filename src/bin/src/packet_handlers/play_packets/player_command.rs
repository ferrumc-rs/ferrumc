use bevy_ecs::prelude::{Query, Res};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::incoming::player_command::PlayerCommandAction;
use ferrumc_net::packets::outgoing::entity_metadata::{EntityMetadata, EntityMetadataPacket};
use ferrumc_net::PlayerCommandPacketReceiver;
use tracing::error;

pub fn handle(events: Res<PlayerCommandPacketReceiver>, query: Query<&StreamWriter>) {
    for (event, _) in events.0.try_iter() {
        match event.action {
            PlayerCommandAction::StartSneaking => {
                let packet = EntityMetadataPacket::new(
                    event.entity_id,
                    [
                        EntityMetadata::entity_sneaking_visual(),
                        EntityMetadata::entity_sneaking_pressed(),
                    ],
                );

                // TODO: Don't clone
                for conn in query {
                    if !conn.running.load(std::sync::atomic::Ordering::Relaxed) {
                        continue;
                    }
                    if let Err(err) = conn.send_packet(packet.clone()) {
                        error!("Failed to send start sneaking packet: {:?}", err);
                    }
                }
            }
            PlayerCommandAction::StopSneaking => {
                let packet =
                    EntityMetadataPacket::new(event.entity_id, [EntityMetadata::entity_standing()]);

                for conn in query {
                    if !conn.running.load(std::sync::atomic::Ordering::Relaxed) {
                        continue;
                    }
                    if let Err(err) = conn.send_packet(packet.clone()) {
                        error!("Failed to send stop sneaking packet: {:?}", err);
                    }
                }
            }
            _ => {}
        }
    }
}
