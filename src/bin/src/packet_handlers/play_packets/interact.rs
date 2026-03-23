use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_components::health::Health;
use ferrumc_core::identity::entity_identity::EntityIdentity;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_core::transform::velocity::Velocity;
use ferrumc_entities::components::combat::CombatProperties;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::incoming::interact::InteractEntity;
use ferrumc_net::packets::outgoing::hurt_animation::HurtAnimationPacket;
use ferrumc_net::InteractEntityReceiver;
use ferrumc_state::GlobalStateResource;
use tracing::error;

/// Standard knockback horizontal strength.
const KNOCKBACK_STRENGTH: f32 = 0.5;
/// Vertical hop applied on knockback.
const KNOCKBACK_HOP: f32 = 0.4;

type InteractEntityQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        Option<&'static EntityIdentity>,
        Option<&'static PlayerIdentity>,
        Option<&'static mut Health>,
        Option<&'static mut CombatProperties>,
        Option<&'static mut Velocity>,
        Option<&'static mut OnGround>,
    ),
>;

pub fn handle(
    receiver: Res<InteractEntityReceiver>,
    mut entity_query: InteractEntityQuery,
    attacker_query: Query<(&Position, &Rotation)>,
    conn_query: Query<(Entity, &StreamWriter)>,
    state: Res<GlobalStateResource>,
) {
    for (event, attacker_eid) in receiver.0.try_iter() {
        if !event.is_attack() {
            continue;
        }

        for (
            _e,
            entity_id_comp,
            player_id_comp,
            mut health,
            mut combat,
            mut velocity,
            mut on_ground,
        ) in entity_query.iter_mut()
        {
            if !is_target(entity_id_comp, player_id_comp, &event) {
                continue;
            }

            let Some(ref mut combat) = combat else {
                continue;
            };

            if !combat.can_be_damaged() {
                continue;
            }

            let Ok((_attacker_pos, attacker_rot)) = attacker_query.get(attacker_eid) else {
                continue;
            };

            // Mark entity as invulnerable for some amount of time after being attacked
            combat.set_default_invulnerability();

            // Decrease health if the entity has a Health component
            // TODO: Ensure death is handled when health reaches 0.
            // TODO: Ensure the held item and armor is taken into account for damage calculation.
            if let Some(ref mut health) = health {
                health.current -= 1.0;
            }

            apply_knockback(
                velocity.as_deref_mut(),
                on_ground.as_deref_mut(),
                attacker_rot,
            );

            broadcast_hurt_animation(&conn_query, &state, &event, attacker_rot);
        }
    }
}

/// Apply knockback to the entity based on the attacker's facing direction.
fn apply_knockback(
    velocity: Option<&mut Velocity>,
    on_ground: Option<&mut OnGround>,
    attacker_rot: &Rotation,
) {
    let Some(v) = velocity else { return };

    let yaw_rad = attacker_rot.yaw.to_radians();

    v.vec.x += -yaw_rad.sin() * KNOCKBACK_STRENGTH;
    v.vec.y += KNOCKBACK_HOP;
    v.vec.z += yaw_rad.cos() * KNOCKBACK_STRENGTH;

    if let Some(og) = on_ground {
        og.0 = false;
    }
}

/// Broadcast the hurt animation packet to all connected players.
fn broadcast_hurt_animation(
    conn_query: &Query<(Entity, &StreamWriter)>,
    state: &Res<GlobalStateResource>,
    event: &InteractEntity,
    attacker_rot: &Rotation,
) {
    let hurt_packet = HurtAnimationPacket::new(event.entity_id.0, attacker_rot.yaw);

    for (conn_entity, conn) in conn_query.iter() {
        if !state.0.players.is_connected(conn_entity) {
            continue;
        }

        if let Err(e) = conn.send_packet_ref(&hurt_packet) {
            error!("Failed to send damage status packet: {}", e);
        }
    }
}

/// Check whether the given entity components match the target of the interact event.
fn is_target(
    entity_id_comp: Option<&EntityIdentity>,
    player_id_comp: Option<&PlayerIdentity>,
    event: &InteractEntity,
) -> bool {
    let target_network_id = event.entity_id.0;

    if let Some(eid) = entity_id_comp {
        eid.entity_id == target_network_id
    } else if let Some(pid) = player_id_comp {
        pid.short_uuid == target_network_id
    } else {
        false
    }
}
