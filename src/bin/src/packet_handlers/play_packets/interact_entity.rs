use bevy_ecs::prelude::{EventWriter, Query, Res};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_entities::DamageEvent;
use ferrumc_entities::EntityNetworkIdIndex;
use ferrumc_net::InteractEntityPacketReceiver;
use ferrumc_state::GlobalStateResource;
use tracing::{debug, warn};

pub fn handle(
    events: Res<InteractEntityPacketReceiver>,
    player_query: Query<(&PlayerIdentity, &Position)>,
    entity_query: Query<&Position>,
    entity_index: Res<EntityNetworkIdIndex>,
    mut damage_events: EventWriter<DamageEvent>,
    _state: Res<GlobalStateResource>,
) {
    for (event, player_eid) in events.0.try_iter() {
        // Get player identity and position
        let Ok((player_identity, player_pos)) = player_query.get(player_eid) else {
            warn!(
                "Player identity/position not found for entity {:?}",
                player_eid
            );
            continue;
        };

        // Check if this is an attack interaction
        if !event.is_attack() {
            debug!(
                "Non-attack interaction type {} (not implemented yet)",
                event.interaction_type.0
            );
            continue;
        }

        // Fast O(1) lookup using network ID index
        let target_network_id = event.entity_id.0;
        let Some(target) = entity_index.get(target_network_id) else {
            warn!(
                "Player {} attacked non-existent entity with network ID {}",
                player_identity.short_uuid, target_network_id
            );
            continue;
        };

        // Get target position
        let Ok(target_pos) = entity_query.get(target) else {
            warn!("Target entity {:?} missing Position component", target);
            continue;
        };

        // Calculate knockback direction (from attacker to target, normalized)
        let dx = target_pos.x - player_pos.x;
        let dz = target_pos.z - player_pos.z;
        let horizontal_dist = (dx * dx + dz * dz).sqrt();

        let knockback_direction = if horizontal_dist > 0.0 {
            Some((dx / horizontal_dist, 0.0, dz / horizontal_dist))
        } else {
            None
        };

        // Base damage for unarmed attack in Minecraft is 1.0
        // TODO: Calculate damage based on held item, enchantments, critical hits, etc.
        let base_damage = 1.0;

        // Base knockback strength (Minecraft default)
        let knockback_strength = 0.4;

        debug!(
            "Player {} attacked entity {:?} (network ID: {}, sneaking: {}) - Damage: {}, Knockback: {:?}",
            player_identity.short_uuid, target, target_network_id, event.sneaking, base_damage, knockback_direction
        );

        // Write damage event
        damage_events.write(DamageEvent {
            target,
            attacker: Some(player_eid),
            damage: base_damage,
            knockback_direction,
            knockback_strength,
        });
    }
}
