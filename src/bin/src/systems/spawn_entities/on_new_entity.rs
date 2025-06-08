use bevy_ecs::entity::Entity;
use bevy_ecs::query::Added;
use bevy_ecs::system::Query;
use ferrumc_core::entities::entity_kind::EntityKind;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_entities::components::Zombie;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::spawn_entity::SpawnEntityPacket;

pub fn broadcast_new_entities(
    query: Query<(Entity, &Zombie), Added<Zombie>>,
    players_query: Query<(Entity, &mut StreamWriter)>,
    transforms: Query<(&Position, &Rotation, &EntityKind)>,
) {
    for (entity, _zombie) in query.iter() {
        tracing::info!("New Zombie spawned: Entity {:?}, Zombie", entity);

        let (pos, rot, kind) = transforms
            .get(entity)
            .unwrap_or_else(|_| panic!("Missing transform for {:?}", entity));

        let packet = SpawnEntityPacket::entity(entity, pos, rot, kind);

        if let Ok(packet) = packet {
            for (player_entity, writer) in players_query.iter() {
                tracing::debug!("Sending SpawnEntityPacket to player: {:?}", player_entity);
                if let Err(e) = writer.send_packet(packet.clone()) {
                    tracing::error!(
                        "Failed to send SpawnEntityPacket to player {:?}: {}",
                        player_entity,
                        e
                    );
                }
            }
        } else {
            tracing::error!("Failed to create SpawnEntityPacket for entity {:?}", entity);
        }
    }
}
