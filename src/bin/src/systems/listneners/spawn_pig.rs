use bevy_ecs::prelude::*;
use ferrumc_core::identity::entity_identity::EntityIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_entities::bundles::PigBundle;
use ferrumc_entities::components::EntityMetadata;
use ferrumc_entities::{pop_spawn_request, EntityType};
use ferrumc_events::SpawnEntityEvent;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::spawn_entity::SpawnEntityPacket;
use tracing::{error, warn};

/// System that processes spawn commands from the queue
pub fn spawn_command_processor(
    query: Query<(&Position, &Rotation)>,
    mut spawn_events: EventWriter<SpawnEntityEvent>,
) {
    // Process all pending spawn requests from the lock-free queue
    while let Some(request) = pop_spawn_request() {
        // Get player position and rotation
        if let Ok((pos, rot)) = query.get(request.player_entity) {
            // Calculate spawn position 2 blocks in front of the player
            let yaw_radians = rot.yaw.to_radians();
            let spawn_pos = Position::new(
                pos.x - (yaw_radians.sin() as f64 * 2.0),
                pos.y,
                pos.z + (yaw_radians.cos() as f64 * 2.0),
            );

            spawn_events.write(SpawnEntityEvent {
                entity_type: request.entity_type,
                position: spawn_pos,
            });
        } else {
            warn!(
                "Failed to get position for entity {:?}",
                request.player_entity
            );
        }
    }
}

/// System that listens for `SpawnEntityEvent` and spawns the entity,
/// then broadcasts the spawn packet.
pub fn handle_spawn_entity(mut events: EventReader<SpawnEntityEvent>, mut commands: Commands) {
    for event in events.read() {
        match event.entity_type {
            EntityType::Pig => {
                // Spawn the pig entity
                let pig_entity = commands.spawn(PigBundle::new(event.position.clone())).id();

                // Queue a deferred system to send packets after pig is spawned
                commands.queue(move |world: &mut World| {
                    // Get pig's components directly from world
                    let metadata = match world.get::<EntityMetadata>(pig_entity) {
                        Some(m) => m,
                        None => {
                            error!("Failed to get pig metadata");
                            return;
                        }
                    };
                    let protocol_id = metadata.protocol_id();

                    let identity = match world.get::<EntityIdentity>(pig_entity) {
                        Some(i) => i,
                        None => {
                            error!("Failed to get pig identity");
                            return;
                        }
                    };

                    let position = match world.get::<Position>(pig_entity) {
                        Some(p) => p,
                        None => {
                            error!("Failed to get pig position");
                            return;
                        }
                    };

                    let rotation = match world.get::<Rotation>(pig_entity) {
                        Some(r) => r,
                        None => {
                            error!("Failed to get pig rotation");
                            return;
                        }
                    };

                    // Create spawn packet using the new method
                    let spawn_packet = SpawnEntityPacket::new(
                        identity.entity_id,
                        identity.uuid.as_u128(),
                        protocol_id as i32,
                        position,
                        rotation,
                    );

                    // Broadcast to all connected players
                    let mut writer_query = world.query::<&StreamWriter>();
                    for writer in writer_query.iter(world) {
                        if let Err(e) = writer.send_packet_ref(&spawn_packet) {
                            error!("Failed to send spawn packet: {:?}", e);
                        }
                    }
                });
            }
        }
    }
}
