use bevy_ecs::prelude::Query;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::incoming::player_command::PlayerCommandAction;
use ferrumc_net::packets::outgoing::entity_metadata::{EntityMetadata, EntityMetadataPacket};
use ferrumc_net::PlayerCommandPacketReceiver;

pub fn handle(events: PlayerCommandPacketReceiver, query: Query<StreamWriter>) {
    for (event, entity) in events.0 {
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
                for stream in query.iter() {
                    stream.send(packet.clone())?;
                }
            }
            PlayerCommandAction::StopSneaking => {
                let packet =
                    EntityMetadataPacket::new(event.entity_id, [EntityMetadata::entity_standing()]);

                for stream in query.iter() {
                    stream.send(packet.clone())?;
                }
            }
            _ => {}
        }
    }
}
