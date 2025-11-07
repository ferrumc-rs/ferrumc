use bevy_ecs::prelude::*;
use ferrumc_entities::components::{EntityId, Health, Velocity};
use ferrumc_entities::DamageEvent;
use ferrumc_state::GlobalStateResource;
use tracing::{debug, info};

/// System that processes damage events and applies damage + knockback to entities
pub fn entity_damage_system(
    mut damage_events: EventReader<DamageEvent>,
    mut entity_query: Query<(&EntityId, &mut Health, &mut Velocity)>,
    _state: Res<GlobalStateResource>,
) {
    for event in damage_events.read() {
        // Get the target entity's components
        let Ok((entity_id, mut health, mut velocity)) = entity_query.get_mut(event.target) else {
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

        // TODO: Send damage animation packet to nearby players
        // TODO: Send hurt sound effect
        // TODO: Apply invulnerability ticks (prevent damage spam)
    }
}
