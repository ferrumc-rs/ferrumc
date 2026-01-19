//! Sneak state component for tracking player sneaking.
//!
//! This component tracks whether a player is currently sneaking, used for:
//! - Broadcasting sneak state changes to other players
//! - Determining entity pose (sneaking vs standing)
//!
//! Sneaking is detected via the PlayerInput packet (flag 0x20) in 1.21.x protocol,
//! NOT via PlayerCommand (which was used in older protocol versions).

use bevy_ecs::prelude::Component;

/// Component tracking whether a player is currently sneaking.
///
/// Updated by the `player_input` packet handler when the sneak flag changes.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct SneakState {
    pub is_sneaking: bool,
}
