//! Unified movement handler for processing all player movement events.
//!
//! This module processes movement packets (position, rotation, or combined) and:
//! - Updates the player's ECS components (Position, Rotation, OnGround)
//! - Detects chunk boundary crossings and sends commands to async chunk loaders
//! - Broadcasts movement updates to other connected players

use bevy_ecs::prelude::{Entity, MessageReader, Query, Res};
use ferrumc_components::chunks::{ChunkCommand, ChunkSender};
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_macros::NetEncode;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::entity_position_sync::TeleportEntityPacket;
use ferrumc_net::packets::outgoing::update_entity_position::UpdateEntityPositionPacket;
use ferrumc_net::packets::outgoing::update_entity_position_and_rotation::UpdateEntityPositionAndRotationPacket;
use ferrumc_net::packets::outgoing::update_entity_rotation::UpdateEntityRotationPacket;
use ferrumc_net::packets::packet_messages::Movement;
use ferrumc_state::GlobalStateResource;
use std::sync::atomic::Ordering;
use tracing::{debug, trace, warn};
use ferrumc_components::player::client_information::ClientInformation;
// ============================================================================
// Constants
// ============================================================================

/// Maximum delta for position updates before teleport is required.
/// As per Minecraft protocol: "If the movement exceeds these limits,
/// Teleport Entity should be sent instead."
const MAX_POSITION_DELTA: i16 = (7.5 * 4096.0_f32) as i16;

/// Scale factor for converting world coordinates to protocol delta values.
const POSITION_SCALE: f64 = 4096.0;

// ============================================================================
// Broadcast Packet Types
// ============================================================================

/// Enum representing all possible movement broadcast packets.
/// Using an enum allows us to handle different packet types uniformly.
#[derive(NetEncode, Clone)]
enum BroadcastMovementPacket {
    UpdateEntityPosition(UpdateEntityPositionPacket),
    UpdateEntityPositionAndRotation(UpdateEntityPositionAndRotationPacket),
    UpdateEntityRotation(UpdateEntityRotationPacket),
    TeleportEntity(TeleportEntityPacket),
}

// ============================================================================
// Main Handler
// ============================================================================

/// Unified movement handler that processes all player movement events.
///
/// This system handles:
/// - Position-only updates (from SetPlayerPosition packets)
/// - Rotation-only updates (from SetPlayerRotation packets)
/// - Combined position+rotation updates (from SetPlayerPositionAndRotation packets)
///
/// For each movement event, it:
/// 1. Validates the player is still connected
/// 2. Calculates position deltas for network updates
/// 3. Detects chunk boundary crossings and notifies async chunk loader
/// 4. Updates ECS components
/// 5. Broadcasts the movement to other players
pub fn handle(
    mut movement_events: MessageReader<Movement>,
    mut transform_query: Query<(
        &mut Position,
        &mut Rotation,
        &mut OnGround,
        &PlayerIdentity,
        &ChunkSender,
    )>,
    client_information: Query<&ClientInformation>,
    conn_query: Query<(Entity, &StreamWriter)>,
    state: Res<GlobalStateResource>,
) {
    for movement in movement_events.read() {
        process_movement_event(&movement, &mut transform_query, &client_information, &conn_query, &state);
    }
}

/// Processes a single movement event.
fn process_movement_event(
    movement: &Movement,
    transform_query: &mut Query<(
        &mut Position,
        &mut Rotation,
        &mut OnGround,
        &PlayerIdentity,
        &ChunkSender,
    )>,
    client_information: &Query<&ClientInformation>,
    conn_query: &Query<(Entity, &StreamWriter)>,
    state: &Res<GlobalStateResource>,
) {
    let entity = movement.entity;

    // Validate player connection
    if !state.0.players.is_connected(entity) {
        trace!("Skipping movement for disconnected player {:?}", entity);
        return;
    }

    // Get transform components
    let Ok((mut position, mut rotation, mut on_ground, identity, chunk_sender)) =
        transform_query.get_mut(entity)
    else {
        debug!("Failed to get transform components for entity {:?}", entity);
        return;
    };

    // Calculate position delta for network broadcast
    let delta_pos = calculate_position_delta(&position, movement.position.as_ref());

    // Handle chunk boundary crossing - send command to async chunk loader
    if let Some(new_pos) = &movement.position {
        if has_crossed_chunk_boundary(&position, new_pos) {
            let chunk_x = new_pos.x.floor() as i32 >> 4;
            let chunk_z = new_pos.z.floor() as i32 >> 4;
            let radius = get_global_config().chunk_render_distance as u8;
            let client_info = client_information.get(entity).ok();
            let radius = client_info
                .map(|info| {
                    let client_view_distance = info.view_distance;
                    let server_render_distance = radius;
                    // Don't send more than what the server allows, nor more than what the client wants
                    server_render_distance.min(client_view_distance)
                })
                .unwrap_or(radius);

            // Use try_send to avoid blocking the ECS tick if the channel is full
            if let Err(e) = chunk_sender.tx.try_send(ChunkCommand::UpdateCenter {
                chunk_x,
                chunk_z,
                radius,
            }) {
                debug!(
                    "Failed to send chunk update command for {:?}: {:?}",
                    entity, e
                );
            }
        }
    }

    // Update ECS components
    update_transform_components(
        &mut position,
        &mut rotation,
        &mut on_ground,
        movement.position.as_ref(),
        movement.rotation.as_ref(),
        movement.on_ground,
    );

    // Build and broadcast movement packet
    if let Some(packet) = build_broadcast_packet(
        delta_pos,
        movement.rotation.as_ref(),
        &position,
        &rotation,
        on_ground.0,
        identity,
    ) {
        broadcast_to_other_players(entity, &packet, conn_query, state);
    }

    trace!(
        "Processed movement for {:?}: pos={:?}, rot={:?}",
        entity,
        movement.position.is_some(),
        movement.rotation.is_some()
    );
}

// ============================================================================
// Position Calculations
// ============================================================================

/// Calculates the position delta between current and new position.
///
/// Returns `None` if no new position is provided.
/// The delta is scaled to protocol units (4096 per block).
#[inline]
fn calculate_position_delta(
    current: &Position,
    new_pos: Option<&Position>,
) -> Option<(i16, i16, i16)> {
    new_pos.map(|new| {
        (
            scale_delta(new.x, current.x),
            scale_delta(new.y, current.y),
            scale_delta(new.z, current.z),
        )
    })
}

/// Scales a coordinate delta to protocol units.
#[inline]
fn scale_delta(new: f64, current: f64) -> i16 {
    ((new * POSITION_SCALE) - (current * POSITION_SCALE)) as i16
}

/// Checks if the position delta exceeds the threshold requiring teleport.
#[inline]
fn delta_exceeds_threshold(delta: Option<(i16, i16, i16)>) -> bool {
    match delta {
        Some((dx, dy, dz)) => {
            // Handle i16::MIN edge case to prevent overflow on abs()
            dx == i16::MIN
                || dy == i16::MIN
                || dz == i16::MIN
                || dx.abs() > MAX_POSITION_DELTA
                || dy.abs() > MAX_POSITION_DELTA
                || dz.abs() > MAX_POSITION_DELTA
        }
        None => false,
    }
}

/// Checks if the player has crossed a chunk boundary.
#[inline]
fn has_crossed_chunk_boundary(old_pos: &Position, new_pos: &Position) -> bool {
    let old_chunk = position_to_chunk(old_pos);
    let new_chunk = (new_pos.x as i32 >> 4, new_pos.z as i32 >> 4);
    old_chunk != new_chunk
}

/// Converts a position to chunk coordinates.
#[inline]
pub fn position_to_chunk(pos: &Position) -> (i32, i32) {
    (pos.x as i32 >> 4, pos.z as i32 >> 4)
}

// ============================================================================
// Component Updates
// ============================================================================

/// Updates the transform components with new values.
#[inline]
fn update_transform_components(
    position: &mut Position,
    rotation: &mut Rotation,
    on_ground: &mut OnGround,
    new_pos: Option<&Position>,
    new_rot: Option<&Rotation>,
    new_ground: Option<bool>,
) {
    if let Some(pos) = new_pos {
        *position = Position::new(pos.x, pos.y, pos.z);
    }

    if let Some(rot) = new_rot {
        *rotation = Rotation::new(rot.yaw, rot.pitch);
    }

    if let Some(ground) = new_ground {
        *on_ground = OnGround(ground);
    }
}

// ============================================================================
// Packet Building
// ============================================================================

/// Builds the appropriate broadcast packet based on what changed.
///
/// Returns `None` if nothing needs to be broadcast (no position or rotation change).
fn build_broadcast_packet(
    delta_pos: Option<(i16, i16, i16)>,
    new_rot: Option<&Rotation>,
    current_pos: &Position,
    current_rot: &Rotation,
    on_ground: bool,
    identity: &PlayerIdentity,
) -> Option<BroadcastMovementPacket> {
    // If delta exceeds threshold, force teleport packet
    if delta_exceeds_threshold(delta_pos) {
        return Some(BroadcastMovementPacket::TeleportEntity(
            TeleportEntityPacket::new(identity, current_pos, current_rot, on_ground),
        ));
    }

    // Build appropriate packet based on available data
    match (delta_pos, new_rot) {
        (Some(delta), Some(rot)) => Some(BroadcastMovementPacket::UpdateEntityPositionAndRotation(
            UpdateEntityPositionAndRotationPacket::new(identity, delta, rot, on_ground),
        )),
        (Some(delta), None) => Some(BroadcastMovementPacket::UpdateEntityPosition(
            UpdateEntityPositionPacket::new(identity, delta, on_ground),
        )),
        (None, Some(rot)) => Some(BroadcastMovementPacket::UpdateEntityRotation(
            UpdateEntityRotationPacket::new(identity, rot, on_ground),
        )),
        (None, None) => None,
    }
}

// ============================================================================
// Broadcasting
// ============================================================================

/// Broadcasts a movement packet to all connected players except the source.
fn broadcast_to_other_players(
    source_entity: Entity,
    packet: &BroadcastMovementPacket,
    conn_query: &Query<(Entity, &StreamWriter)>,
    state: &Res<GlobalStateResource>,
) {
    for (entity, conn) in conn_query.iter() {
        // Skip the player who moved
        if entity == source_entity {
            continue;
        }

        // Skip disconnected players
        if !is_player_connected(entity, conn, state) {
            handle_disconnected_player(entity, state);
            continue;
        }

        // Send the packet
        if let Err(e) = conn.send_packet_ref(packet) {
            warn!("Failed to send movement packet to {:?}: {}", entity, e);
        }
    }
}

/// Checks if a player is still connected.
#[inline]
fn is_player_connected(
    entity: Entity,
    conn: &StreamWriter,
    state: &Res<GlobalStateResource>,
) -> bool {
    state.0.players.is_connected(entity) && conn.running.load(Ordering::Relaxed)
}

/// Handles cleanup for a disconnected player.
fn handle_disconnected_player(entity: Entity, state: &Res<GlobalStateResource>) {
    warn!("Player {:?} is not connected, cleaning up", entity);
    state
        .0
        .players
        .disconnect(entity, Some(String::from("Player not connected anymore.")));
}
