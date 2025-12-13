//! Debug display system for rendering debug information to player action bars.
//!
//! This system runs each tick and sends action bar messages to players who have
//! debug display enabled. The information shown depends on which debug flags
//! the player has enabled via the `/debug` command.

use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_components::player::debug_settings::{DebugFlags, DebugSettings};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::system_message::SystemMessagePacket;
use ferrumc_state::GlobalStateResource;
use ferrumc_text::{NamedColor, TextComponent, TextComponentBuilder};
use std::sync::atomic::Ordering;
use tracing::warn;

/// System that sends debug information to players with debug display enabled.
///
/// This system iterates over all players with debug settings and sends an action bar
/// message containing the requested debug information. The action bar is continuously
/// updated each tick to show real-time data.
pub fn handle(
    debug_query: Query<(
        Entity,
        &DebugSettings,
        &Position,
        &Rotation,
        &PlayerIdentity,
        &StreamWriter,
    )>,
    state: Res<GlobalStateResource>,
) {
    for (entity, debug_settings, position, rotation, _identity, writer) in debug_query.iter() {
        // Skip players with no debug flags enabled
        if !debug_settings.has_any_enabled() {
            continue;
        }

        // Skip disconnected players
        if !state.0.players.is_connected(entity) || !writer.running.load(Ordering::Relaxed) {
            continue;
        }

        // Build the debug message based on enabled flags
        let message = build_debug_message(debug_settings, position, rotation);

        // Send as action bar (overlay = true)
        let packet = SystemMessagePacket::new(message, true);

        if let Err(e) = writer.send_packet_ref(&packet) {
            warn!(
                "Failed to send debug action bar to player {:?}: {}",
                entity, e
            );
        }
    }
}

/// Builds a debug message based on the player's enabled debug flags.
///
/// The message is formatted with colors to make it easy to read in the action bar.
fn build_debug_message(
    settings: &DebugSettings,
    position: &Position,
    rotation: &Rotation,
) -> TextComponent {
    let mut parts: Vec<TextComponent> = Vec::new();
    let flags = settings.flags();

    // Chunk information
    if flags.contains(DebugFlags::CHUNK_INFO) {
        let chunk_x = position.x as i32 >> 4;
        let chunk_z = position.z as i32 >> 4;
        parts.push(format_debug_section("Chunk", &format!("{}, {}", chunk_x, chunk_z)));
    }

    // Position information
    if flags.contains(DebugFlags::POSITION_INFO) {
        parts.push(format_debug_section(
            "XYZ",
            &format!("{:.1}, {:.1}, {:.1}", position.x, position.y, position.z),
        ));
    }

    // Rotation information
    if flags.contains(DebugFlags::ROTATION_INFO) {
        parts.push(format_debug_section(
            "Rot",
            &format!("{:.1}° / {:.1}°", rotation.yaw, rotation.pitch),
        ));
    }

    // Join all parts with separators
    if parts.is_empty() {
        return TextComponentBuilder::new("Debug enabled (no flags selected)")
            .color(NamedColor::Gray)
            .build();
    }

    // Build the final message by joining with separators
    join_debug_parts(parts)
}

/// Formats a single debug section with label and value.
fn format_debug_section(label: &str, value: &str) -> TextComponent {
    TextComponentBuilder::new(label)
        .color(NamedColor::Gray)
        .extra(
            TextComponentBuilder::new(": ")
                .color(NamedColor::DarkGray)
                .build(),
        )
        .extra(
            TextComponentBuilder::new(value)
                .color(NamedColor::White)
                .build(),
        )
        .build()
}

/// Joins multiple debug parts with a separator.
fn join_debug_parts(parts: Vec<TextComponent>) -> TextComponent {
    let separator = TextComponentBuilder::new(" | ")
        .color(NamedColor::DarkGray)
        .build();

    let mut result = TextComponent::default();
    let mut first = true;

    for part in parts {
        if !first {
            result.extra.push(separator.clone());
        }
        result.extra.push(part);
        first = false;
    }

    result
}

// ============================================================================
// Utility Functions for Debug Information
// ============================================================================
// These utility functions are provided for future expansion of the debug system.
// They can be used by other modules or enabled debug flags as needed.

/// Calculates the chunk coordinates from a world position.
///
/// # Arguments
/// * `x` - World X coordinate
/// * `z` - World Z coordinate
///
/// # Returns
/// A tuple of (chunk_x, chunk_z)
#[inline]
#[allow(dead_code)]
pub fn world_to_chunk_coords(x: f64, z: f64) -> (i32, i32) {
    (x as i32 >> 4, z as i32 >> 4)
}

/// Calculates the block coordinates within a chunk from a world position.
///
/// # Arguments
/// * `x` - World X coordinate
/// * `z` - World Z coordinate
///
/// # Returns
/// A tuple of (block_x, block_z) within the chunk (0-15)
#[inline]
#[allow(dead_code)]
pub fn world_to_block_in_chunk(x: f64, z: f64) -> (i32, i32) {
    let block_x = (x as i32).rem_euclid(16);
    let block_z = (z as i32).rem_euclid(16);
    (block_x, block_z)
}

/// Formats a position for display with the specified precision.
#[inline]
#[allow(dead_code)]
pub fn format_position(position: &Position, precision: usize) -> String {
    format!(
        "{:.prec$}, {:.prec$}, {:.prec$}",
        position.x,
        position.y,
        position.z,
        prec = precision
    )
}

/// Formats rotation angles for display.
#[inline]
#[allow(dead_code)]
pub fn format_rotation(rotation: &Rotation) -> String {
    format!("{:.1}° yaw, {:.1}° pitch", rotation.yaw, rotation.pitch)
}

/// Gets the cardinal direction name from a yaw angle.
#[allow(dead_code)]
pub fn yaw_to_direction(yaw: f32) -> &'static str {
    // Normalize yaw to 0-360 range
    let normalized = ((yaw % 360.0) + 360.0) % 360.0;

    match normalized {
        y if y >= 337.5 || y < 22.5 => "South",
        y if y >= 22.5 && y < 67.5 => "Southwest",
        y if y >= 67.5 && y < 112.5 => "West",
        y if y >= 112.5 && y < 157.5 => "Northwest",
        y if y >= 157.5 && y < 202.5 => "North",
        y if y >= 202.5 && y < 247.5 => "Northeast",
        y if y >= 247.5 && y < 292.5 => "East",
        y if y >= 292.5 && y < 337.5 => "Southeast",
        _ => "Unknown",
    }
}
