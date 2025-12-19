use bevy_ecs::prelude::*;
use ferrumc_core::identity::entity_identity::EntityIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_entities::bundles::PigBundle;
use ferrumc_entities::components::EntityMetadata;
use ferrumc_entities::markers::entity_types::Pig;
use ferrumc_messages::{EntityType, SpawnEntityCommand, SpawnEntityEvent};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::spawn_entity::SpawnEntityPacket;
use tracing::{error, warn};

/// Helper function to broadcast entity spawn packets to all connected players.
///
/// This function queries the entity's components and sends the spawn packet
/// to all players. It's generic and works for any entity type.
///
/// # Arguments
///
/// * `world` - The Bevy world
/// * `entity` - The entity to broadcast
fn broadcast_entity_spawn(world: &mut World, entity: Entity) {
    // Get entity components
    let metadata = match world.get::<EntityMetadata>(entity) {
        Some(m) => m,
        None => {
            error!("Failed to get entity metadata for {:?}", entity);
            return;
        }
    };
    let protocol_id = metadata.protocol_id();

    let identity = match world.get::<EntityIdentity>(entity) {
        Some(i) => i,
        None => {
            error!("Failed to get entity identity for {:?}", entity);
            return;
        }
    };

    let position = match world.get::<Position>(entity) {
        Some(p) => p,
        None => {
            error!("Failed to get entity position for {:?}", entity);
            return;
        }
    };

    let rotation = match world.get::<Rotation>(entity) {
        Some(r) => r,
        None => {
            error!("Failed to get entity rotation for {:?}", entity);
            return;
        }
    };

    // Create spawn packet
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
}

/// System that processes spawn commands from messages
pub fn spawn_command_processor(
    mut spawn_commands: MessageReader<SpawnEntityCommand>,
    query: Query<(&Position, &Rotation)>,
    mut spawn_events: MessageWriter<SpawnEntityEvent>,
) {
    // Process all spawn command messages
    for command in spawn_commands.read() {
        // Get player position and rotation
        if let Ok((pos, rot)) = query.get(command.player_entity) {
            // Calculate spawn position 2 blocks in front of the player
            let spawn_pos = pos.offset_forward(rot, 2.0);

            spawn_events.write(SpawnEntityEvent {
                entity_type: command.entity_type,
                position: spawn_pos,
            });
        } else {
            warn!(
                "Failed to get position for entity {:?}",
                command.player_entity
            );
        }
    }
}

/// System that listens for `SpawnEntityEvent` and spawns the entity,
/// then broadcasts the spawn packet.
pub fn handle_spawn_entity(mut events: MessageReader<SpawnEntityEvent>, mut commands: Commands) {
    for event in events.read() {
        match event.entity_type {
            EntityType::Pig => {
                // Spawn the pig entity
                let pig_entity = commands
                    .spawn((PigBundle::new(event.position.clone()), Pig))
                    .id();

                // Queue a deferred system to broadcast spawn packets after entity is fully spawned
                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, pig_entity);
                });
            }
        }
    }
}
