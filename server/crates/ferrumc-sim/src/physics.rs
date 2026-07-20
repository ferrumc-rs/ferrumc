//! Per-tick physics constants for non-player entity motion.
//!
//! Gravity magnitudes and air drag live here, in the simulation crate, rather
//! than in `ferrumc-math`: that crate is pure geometry (coordinates, vectors,
//! bounding boxes) and has no notion of game physics. A gravity value is a
//! *simulation rule*, not a mathematical primitive, so it belongs beside the
//! shard that applies it — and keeping it here honours the one-crate-per-PR
//! boundary.
//!
//! # Per-category gravity
//!
//! Vanilla applies a different downward acceleration (in blocks per tick²) to
//! each broad entity category. A single global constant is *not* vanilla-correct
//! — an arrow falls faster than a snowball, and both fall slower than a mob.
//! Gravity is therefore stored per entity (see the shard's entity store), seeded
//! at spawn from one of these constants:
//!
//! | Category | Constant | Value |
//! | --- | --- | --- |
//! | Living entities (mobs, players) | [`GRAVITY_LIVING`] | `-0.08` |
//! | Items, falling blocks, primed TNT, boats | [`GRAVITY_ITEM`] | `-0.04` |
//! | Arrows, tridents | [`GRAVITY_ARROW`] | `-0.05` |
//! | Thrown items, experience orbs | [`GRAVITY_THROWN`] | `-0.03` |
//!
//! # Terminal velocity
//!
//! Vanilla does not clamp fall speed to a constant; it multiplies the vertical
//! velocity by a drag factor every tick. With gravity `g` (negative) applied
//! before integration and drag `d` = [`AIR_DRAG_Y`] applied after, the vertical
//! velocity converges to the fixed point of `v → (v + g) · d`:
//!
//! ```text
//!   v = (v + g)·d  ⇒  v = g·d / (1 − d)
//! ```
//!
//! For living entities that is `-0.08 · 0.98 / 0.02 = -3.92` blocks/tick — the
//! same terminal velocity the v1 server hardcoded, here emerging from the drag
//! rather than a magic clamp.

/// Downward acceleration for living entities (mobs, players), blocks per tick².
pub const GRAVITY_LIVING: f64 = -0.08;

/// Downward acceleration for items, falling blocks, primed TNT, and boats.
pub const GRAVITY_ITEM: f64 = -0.04;

/// Downward acceleration for arrows and tridents.
pub const GRAVITY_ARROW: f64 = -0.05;

/// Downward acceleration for thrown items (snowball, egg, ender pearl) and
/// experience orbs.
pub const GRAVITY_THROWN: f64 = -0.03;

/// Per-tick multiplier applied to the vertical velocity after integration.
///
/// Models air resistance on the fall axis. Combined with a per-tick gravity
/// impulse this yields a terminal velocity of `gravity · AIR_DRAG_Y / (1 −
/// AIR_DRAG_Y)` without a hardcoded clamp — see the module docs.
pub const AIR_DRAG_Y: f64 = 0.98;

/// Returns `true` if a block with the bare resource name `block_name` falls when
/// it loses support — the vanilla `FallingBlock` set.
///
/// `block_name` is the bare name (no `minecraft:` namespace), as returned by
/// [`ferrumc_registry::block_state::state_id_to_block_name`]. This predicate
/// lives here, keyed by name, because the trait is *behavioural* (a code-side
/// `FallingBlock` class in vanilla) and is not present in the vendored
/// `blocks.json` the registry is built from.
///
/// # Coverage
///
/// The current 1.21.8 `FallingBlock` set, minus `scaffolding` (which has its own
/// lateral-support rules and is deferred): sands, gravels, all concrete powders,
/// anvils, the dragon egg, and pointed dripstone. Extend the match as new falling
/// blocks are supported.
#[must_use]
pub fn is_gravity_affected(block_name: &str) -> bool {
    matches!(
        block_name,
        "sand"
            | "red_sand"
            | "gravel"
            | "suspicious_sand"
            | "suspicious_gravel"
            | "anvil"
            | "chipped_anvil"
            | "damaged_anvil"
            | "dragon_egg"
            | "pointed_dripstone"
    ) || block_name.ends_with("_concrete_powder")
}

/// Returns `true` if a falling block coming to rest in a cell already occupied by
/// this block breaks instead of landing — the vanilla `FallingBlock` rule that a
/// block cannot settle onto a non-full "object" (a torch, pressure plate, button,
/// lever, rail, or redstone component).
///
/// `block_name` is the bare name (no `minecraft:` namespace). The check is on the
/// *resting cell* only (the cell directly above the support block), so a wall
/// torch attached to a side block — which lives in a different cell — never breaks
/// a block falling down an adjacent column.
///
/// # Coverage
///
/// The clear non-replaceable objects a falling block breaks on. Replaceable
/// growth (short grass, ferns, seagrass) and fluids are *not* here — a block
/// settles through them. A full `replaceable` classification (to also break on
/// flowers, dead bushes, …) needs a block property the vendored `blocks.json`
/// does not carry, so this stays a focused, name-keyed set like
/// [`is_gravity_affected`]. Extend as needed.
#[must_use]
pub fn breaks_falling_block(block_name: &str) -> bool {
    matches!(
        block_name,
        "torch"
            | "wall_torch"
            | "soul_torch"
            | "soul_wall_torch"
            | "redstone_torch"
            | "redstone_wall_torch"
            | "lever"
            | "rail"
            | "powered_rail"
            | "detector_rail"
            | "activator_rail"
            | "redstone_wire"
            | "repeater"
            | "comparator"
            | "tripwire"
            | "tripwire_hook"
    ) || block_name.ends_with("_pressure_plate")
        || block_name.ends_with("_button")
}

#[cfg(test)]
mod tests {
    use super::{breaks_falling_block, is_gravity_affected};

    #[test]
    fn known_falling_blocks_are_affected() {
        for name in [
            "sand",
            "red_sand",
            "gravel",
            "anvil",
            "dragon_egg",
            "pointed_dripstone",
            "white_concrete_powder",
            "black_concrete_powder",
        ] {
            assert!(is_gravity_affected(name), "{name} should fall");
        }
    }

    #[test]
    fn ordinary_blocks_are_not_affected() {
        for name in ["stone", "grass_block", "oak_log", "concrete", "sandstone"] {
            assert!(!is_gravity_affected(name), "{name} should not fall");
        }
    }

    #[test]
    fn objects_break_a_falling_block() {
        for name in [
            "torch",
            "wall_torch",
            "soul_torch",
            "redstone_torch",
            "lever",
            "rail",
            "powered_rail",
            "stone_pressure_plate",
            "oak_pressure_plate",
            "stone_button",
            "oak_button",
        ] {
            assert!(
                breaks_falling_block(name),
                "{name} should break a falling block"
            );
        }
    }

    #[test]
    fn air_replaceable_and_full_blocks_do_not_break() {
        // Air, replaceable growth, and ordinary solids are all fine to settle onto
        // or through — none breaks a falling block.
        for name in [
            "air",
            "short_grass",
            "tall_grass",
            "fern",
            "water",
            "stone",
            "sand",
        ] {
            assert!(
                !breaks_falling_block(name),
                "{name} should not break a falling block"
            );
        }
    }
}
