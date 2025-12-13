//! Player debug settings component.
//!
//! This module provides a component for storing per-player debug display preferences.
//! Debug information can be toggled via commands and is displayed in the action bar.

use bevy_ecs::prelude::Component;
use std::fmt;

/// Flags representing different debug display options.
///
/// Uses bitflags for efficient storage and easy combination of multiple debug modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DebugFlags(u32);

impl DebugFlags {
    /// No debug information displayed.
    pub const NONE: Self = Self(0);
    /// Display chunk coordinates (chunk X, chunk Z).
    pub const CHUNK_INFO: Self = Self(1 << 0);
    /// Display exact position coordinates (X, Y, Z).
    pub const POSITION_INFO: Self = Self(1 << 1);
    /// Display rotation information (yaw, pitch).
    pub const ROTATION_INFO: Self = Self(1 << 2);
    /// Display all debug information.
    pub const ALL: Self = Self(0xFFFF_FFFF);

    /// Creates a new `DebugFlags` with the given raw value.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Returns the raw bits of the flags.
    #[inline]
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Returns true if no flags are set.
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns true if the specified flag is set.
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Sets the specified flag.
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }

    /// Clears the specified flag.
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.0;
    }

    /// Toggles the specified flag.
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.0;
    }

    /// Returns the union of two flag sets.
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Returns the intersection of two flag sets.
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}

impl std::ops::BitOr for DebugFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl std::ops::BitOrAssign for DebugFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.insert(rhs);
    }
}

impl std::ops::BitAnd for DebugFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl fmt::Display for DebugFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "none");
        }

        let mut parts = Vec::new();
        if self.contains(Self::CHUNK_INFO) {
            parts.push("chunk");
        }
        if self.contains(Self::POSITION_INFO) {
            parts.push("position");
        }
        if self.contains(Self::ROTATION_INFO) {
            parts.push("rotation");
        }
        write!(f, "{}", parts.join(", "))
    }
}

/// Component storing a player's debug display preferences.
///
/// This component is attached to player entities and tracks which debug
/// information should be displayed in their action bar.
///
/// # Example
///
/// ```ignore
/// use ferrumc_components::player::debug_settings::{DebugSettings, DebugFlags};
///
/// let mut settings = DebugSettings::default();
/// settings.enable(DebugFlags::CHUNK_INFO);
/// assert!(settings.is_enabled(DebugFlags::CHUNK_INFO));
/// ```
#[derive(Component, Debug, Clone, Default)]
pub struct DebugSettings {
    /// Currently enabled debug flags.
    flags: DebugFlags,
}

impl DebugSettings {
    /// Creates new debug settings with no flags enabled.
    #[inline]
    pub const fn new() -> Self {
        Self {
            flags: DebugFlags::NONE,
        }
    }

    /// Creates debug settings with the specified flags enabled.
    #[inline]
    pub const fn with_flags(flags: DebugFlags) -> Self {
        Self { flags }
    }

    /// Returns the current debug flags.
    #[inline]
    pub const fn flags(&self) -> DebugFlags {
        self.flags
    }

    /// Returns true if any debug display is enabled.
    #[inline]
    pub const fn has_any_enabled(&self) -> bool {
        !self.flags.is_empty()
    }

    /// Returns true if the specified debug flag is enabled.
    #[inline]
    pub const fn is_enabled(&self, flag: DebugFlags) -> bool {
        self.flags.contains(flag)
    }

    /// Enables the specified debug flag.
    #[inline]
    pub fn enable(&mut self, flag: DebugFlags) {
        self.flags.insert(flag);
    }

    /// Disables the specified debug flag.
    #[inline]
    pub fn disable(&mut self, flag: DebugFlags) {
        self.flags.remove(flag);
    }

    /// Toggles the specified debug flag.
    /// Returns the new state (true if now enabled, false if disabled).
    #[inline]
    pub fn toggle(&mut self, flag: DebugFlags) -> bool {
        self.flags.toggle(flag);
        self.flags.contains(flag)
    }

    /// Disables all debug flags.
    #[inline]
    pub fn disable_all(&mut self) {
        self.flags = DebugFlags::NONE;
    }

    /// Enables all debug flags.
    #[inline]
    pub fn enable_all(&mut self) {
        self.flags = DebugFlags::ALL;
    }
}

/// Parses a debug flag name string into a `DebugFlags` value.
///
/// # Arguments
/// * `name` - The flag name (case-insensitive). Valid values: "chunk", "position", "rotation", "all"
///
/// # Returns
/// * `Some(DebugFlags)` if the name is valid
/// * `None` if the name is not recognized
pub fn parse_debug_flag(name: &str) -> Option<DebugFlags> {
    match name.to_lowercase().as_str() {
        "chunk" | "chunks" => Some(DebugFlags::CHUNK_INFO),
        "position" | "pos" | "coords" => Some(DebugFlags::POSITION_INFO),
        "rotation" | "rot" | "angle" => Some(DebugFlags::ROTATION_INFO),
        "all" => Some(DebugFlags::ALL),
        _ => None,
    }
}

/// Returns a list of all valid debug flag names for help messages.
pub fn available_debug_flags() -> &'static [&'static str] {
    &["chunk", "position", "rotation", "all"]
}
