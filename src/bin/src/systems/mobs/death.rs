use bevy_ecs::prelude::{Commands, Entity, Query, Res, Without};
use ferrumc_components::health::Health;
use ferrumc_core::identity::entity_identity::EntityIdentity;
use ferrumc_entities::components::combat::{CombatProperties, Dying};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::entity_event::EntityStatus;
use ferrumc_net::packets::outgoing::remove_entities::RemoveEntitiesPacket;
use ferrumc_net_codec::encode::NetEncode;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use tracing::error;

/// Vanilla "entity died" code for the Entity Event packet; the client plays the death animation and
/// sound on receipt.
const ENTITY_STATUS_DEATH: u8 = 3;

/// Detects entities whose health has reached zero and starts their death sequence.
///
/// The death animation is broadcast and the entity is tagged [`Dying`] rather than removed
/// immediately, so the client has time to render the animation before [`tick_dying`] despawns it.
/// Only entities carrying an [`EntityIdentity`] are considered, which excludes players (their death
/// is handled separately and they must not be despawned).
pub fn detect_deaths(
    mut commands: Commands,
    mut query: Query<(Entity, &EntityIdentity, &Health, &mut CombatProperties), Without<Dying>>,
    conn_query: Query<(Entity, &StreamWriter)>,
    state: Res<GlobalStateResource>,
) {
    for (entity, identity, health, mut combat) in query.iter_mut() {
        if health.current > 0.0 {
            continue;
        }

        // Prevent any further hits from re-triggering death handling while the animation plays.
        combat.attackable = false;

        let packet = EntityStatus {
            entity_id: identity.entity_id,
            status: ENTITY_STATUS_DEATH,
        };
        broadcast(&conn_query, &state, &packet);

        commands.entity(entity).insert(Dying::default());
    }
}

/// Counts down the death animation for [`Dying`] entities and removes them once it elapses.
///
/// When the timer reaches zero the entity is despawned and a single [`RemoveEntitiesPacket`] covering
/// every entity removed this tick is broadcast to all connected players.
pub fn tick_dying(
    mut commands: Commands,
    mut query: Query<(Entity, &EntityIdentity, &mut Dying)>,
    conn_query: Query<(Entity, &StreamWriter)>,
    state: Res<GlobalStateResource>,
) {
    let mut removed: Vec<VarInt> = Vec::new();

    for (entity, identity, mut dying) in query.iter_mut() {
        if dying.ticks_remaining > 0 {
            dying.ticks_remaining -= 1;
            continue;
        }

        removed.push(VarInt::new(identity.entity_id));
        commands.entity(entity).despawn();
    }

    if removed.is_empty() {
        return;
    }

    let packet = RemoveEntitiesPacket {
        entity_ids: LengthPrefixedVec::new(removed),
    };
    broadcast(&conn_query, &state, &packet);
}

/// Sends a packet to every connected player.
fn broadcast<P: NetEncode + Send>(
    conn_query: &Query<(Entity, &StreamWriter)>,
    state: &Res<GlobalStateResource>,
    packet: &P,
) {
    for (conn_entity, conn) in conn_query.iter() {
        if !state.0.players.is_connected(conn_entity) {
            continue;
        }

        if let Err(e) = conn.send_packet_ref(packet) {
            error!("Failed to broadcast entity death packet: {}", e);
        }
    }
}
