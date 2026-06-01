//! Logical dimensions exposed to gameplay systems.
//!
//! The chunk store and database layer key everything by a free-form `&str` (`"overworld"`,
//! `"the_nether"`, ...) and that is intentionally not changing here: those layers only need a
//! stable identifier and never need to branch on which dimension is which. Gameplay systems
//! (fluids, weather, mob spawning, ...) on the other hand do need to make per-dimension
//! decisions, and a stringly-typed value at every match site is fragile.
//!
//! [`Dimension`] is the small typed projection the gameplay side uses. It converts to/from the
//! string form via [`Dimension::as_str`] / [`Dimension::from_str`] when crossing into the storage
//! layer. New dimensions added to the world should be added here so the compiler points out the
//! match sites that need to learn about them.

/// The vanilla dimensions. Custom dimensions are not modelled yet; if a chunk's dimension
/// string is unrecognised, callers receive `None` from [`Dimension::from_str`] and should fall
/// back to overworld behaviour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dimension {
    Overworld,
    Nether,
    End,
}

impl Dimension {
    /// Returns the canonical string identifier used by the chunk store and network protocol.
    pub const fn as_str(self) -> &'static str {
        match self {
            Dimension::Overworld => "overworld",
            Dimension::Nether => "the_nether",
            Dimension::End => "the_end",
        }
    }

    /// Parses a dimension identifier as it appears in chunk metadata or registry packets.
    ///
    /// Accepts both the bare form (`"overworld"`) and the namespaced form
    /// (`"minecraft:overworld"`) so callers do not need to strip the namespace themselves.
    pub fn from_str(s: &str) -> Option<Self> {
        let bare = s.strip_prefix("minecraft:").unwrap_or(s);
        match bare {
            "overworld" => Some(Self::Overworld),
            "the_nether" | "nether" => Some(Self::Nether),
            "the_end" | "end" => Some(Self::End),
            _ => None,
        }
    }
}

impl Default for Dimension {
    fn default() -> Self {
        Dimension::Overworld
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_known_dimensions() {
        for dim in [Dimension::Overworld, Dimension::Nether, Dimension::End] {
            assert_eq!(Dimension::from_str(dim.as_str()), Some(dim));
        }
    }

    #[test]
    fn accepts_namespaced_form() {
        assert_eq!(
            Dimension::from_str("minecraft:the_nether"),
            Some(Dimension::Nether)
        );
    }

    #[test]
    fn unknown_dimension_is_none() {
        assert_eq!(Dimension::from_str("custom:moon"), None);
    }
}
