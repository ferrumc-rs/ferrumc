use bevy_ecs::prelude::*;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_entities::components::{EntityId, EntityType, Health, Velocity};
use ferrumc_entities::DamageEvent;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::entity_sound_effect::EntitySoundEffectPacket;
use ferrumc_state::GlobalStateResource;
use tracing::{debug, info};

/// System that processes damage events and applies damage + knockback to entities
pub fn entity_damage_system(
    mut damage_events: EventReader<DamageEvent>,
    mut entity_query: Query<(&EntityId, &EntityType, &mut Health, &mut Velocity)>,
    player_query: Query<(Entity, &StreamWriter), With<PlayerIdentity>>,
    state: Res<GlobalStateResource>,
) {
    for event in damage_events.read() {
        // Get the target entity's components
        let Ok((entity_id, entity_type, mut health, mut velocity)) =
            entity_query.get_mut(event.target)
        else {
            debug!(
                "Damage event target entity {:?} not found or missing components",
                event.target
            );
            continue;
        };

        // Apply damage
        health.damage(event.damage);

        info!(
            "Entity {} took {} damage (HP: {}/{})",
            entity_id.to_network_id(),
            event.damage,
            health.current,
            health.max
        );

        // Apply knockback if direction is specified
        if let Some((kx, _ky, kz)) = event.knockback_direction {
            let strength = event.knockback_strength;
            velocity.x += kx * strength;
            velocity.y += 0.4; // Minecraft vanilla: always add upward velocity on hit
            velocity.z += kz * strength;

            debug!(
                "Applied knockback to entity {}: velocity ({:.2}, {:.2}, {:.2})",
                entity_id.to_network_id(),
                velocity.x,
                velocity.y,
                velocity.z
            );
        }

        // Send hurt sound effect to all connected players
        let sound_id = match entity_type {
            EntityType::Pig => 1114, // entity.pig.hurt
            // TODO: Add more entity types and their hurt sounds
            _ => {
                debug!("No hurt sound defined for {:?}", entity_type);
                continue; // Skip sound if not defined
            }
        };

        let sound_packet = EntitySoundEffectPacket::hurt(sound_id, entity_id.to_network_id());

        for (player_entity, stream_writer) in player_query.iter() {
            if state.0.players.is_connected(player_entity) {
                if let Err(e) = stream_writer.send_packet_ref(&sound_packet) {
                    debug!("Failed to send hurt sound to player: {}", e);
                }
            }
        }

        debug!(
            "Sent hurt sound {} for entity {}",
            sound_id,
            entity_id.to_network_id()
        );

        // TODO: Send damage animation packet to nearby players
        // TODO: Apply invulnerability ticks (prevent damage spam)
    }
}
