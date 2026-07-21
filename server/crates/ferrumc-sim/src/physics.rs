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

/// Returns `true` if `block_name` is *replaceable* — a fluid or soft growth that a
/// block can be placed straight into.
///
/// This is the "free" half of vanilla's `FallingBlock::isFree`, and it drives two
/// symmetric rules (see [`is_gravity_affected`] for the falling set):
///
/// - **Support:** a *placed* gravity block falls only when the block below is air
///   (checked separately) or replaceable. Anything else — full cubes, but also
///   non-full blocks like slabs, fences, redstone, torches, saplings, signs —
///   supports it, so it stays put.
/// - **Break:** a *falling* gravity block that comes to rest in a cell holding a
///   non-air, non-replaceable block (a torch, sapling, sign, flower, rail, …)
///   breaks instead of settling; if the cell is air or replaceable it settles,
///   replacing a fluid or plant.
///
/// So keying both rules off this one small set means every non-replaceable object
/// breaks a faller and supports a placement, without enumerating each. `block_name`
/// is the bare name (no `minecraft:` namespace). The vendored `blocks.json` carries
/// no `replaceable` flag, so this is a focused name-keyed set; extend as new
/// replaceable blocks are needed.
#[must_use]
pub fn is_replaceable(block_name: &str) -> bool {
    matches!(
        block_name,
        // Air variants.
        "cave_air"
            | "void_air"
            // Fluids.
            | "water"
            | "lava"
            // Grass and ferns.
            | "short_grass"
            | "tall_grass"
            | "fern"
            | "large_fern"
            // Seagrass.
            | "seagrass"
            | "tall_seagrass"
            // Fire.
            | "fire"
            | "soul_fire"
            // Snow layer (the full `snow_block` is not this).
            | "snow"
            // Vines and multiface growth.
            | "vine"
            | "glow_lichen"
            | "sculk_vein"
            // Roots and sprouts.
            | "hanging_roots"
            | "warped_roots"
            | "crimson_roots"
            | "nether_sprouts"
            // Misc.
            | "structure_void"
    )
}

/// Returns `true` if `block_name` is a *solid but not full-height* block — one a
/// falling block lands on top of (its collision stops the fall) yet cannot settle
/// atop, so the faller breaks instead of placing, matching vanilla.
///
/// Vanilla decides this from the block's collision shape: a falling block's feet come
/// to rest at the block's collision top, and if that top is below a full cube the
/// feet end up *inside* the block's own cell (a non-replaceable cell) and the block
/// breaks. The vendored `blocks.json` records only a coarse `boundingBox` (`"block"`
/// vs `"empty"`) with no per-shape height, so a full cube (stone) and a half-height
/// solid (slab, soul sand, farmland) both read as `"block"` and are indistinguishable
/// by that flag. This name-keyed set names the common `"block"`-box solids whose
/// collision top is below `1.0`, which a faller breaks on.
///
/// Slabs are **not** listed here: their `double` state is a genuine full cube, so a
/// slab is resolved at the call site (see `lands_and_breaks_on_support` in the shard)
/// where the state's `type` property is available. Blocks with an `"empty"` box
/// (torch, rail, sign) are handled by the resting-cell test instead — a faller falls
/// *through* them onto the solid below, leaving the object in its resting cell. This
/// set is name-keyed like [`is_gravity_affected`] and [`is_replaceable`]; extend it as
/// coverage grows. `block_name` is the bare name (no `minecraft:` namespace).
#[must_use]
pub fn is_partial_solid_support(block_name: &str) -> bool {
    matches!(
        block_name,
        // Non-full natural/soil blocks (collision top below 1.0).
        "soul_sand"
            | "mud"
            | "dirt_path"
            | "farmland"
            | "cactus"
            | "honey_block"
            // Speleothems and amethyst growth (pointed, non-full collision).
            | "pointed_dripstone"
            | "amethyst_cluster"
            | "small_amethyst_bud"
            | "medium_amethyst_bud"
            | "large_amethyst_bud"
            // Containers and utility blocks with a low collision top.
            | "chest"
            | "trapped_chest"
            | "ender_chest"
            | "hopper"
            | "cauldron"
            | "water_cauldron"
            | "lava_cauldron"
            | "powder_snow_cauldron"
            | "composter"
            | "brewing_stand"
            | "enchanting_table"
            | "stonecutter"
            | "grindstone"
            | "lectern"
            | "daylight_detector"
            | "cake"
            | "campfire"
            | "soul_campfire"
            | "sculk_sensor"
            | "sculk_shrieker"
            | "conduit"
            | "bell"
    ) || block_name.ends_with("_bed")
}

#[cfg(test)]
mod tests {
    use super::{is_gravity_affected, is_partial_solid_support, is_replaceable};

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
    fn fluids_and_soft_growth_are_replaceable() {
        // A falling block settles through these (and a placed block over them
        // falls): fluids, grass/ferns, seagrass, fire, snow layer, vines, roots.
        for name in [
            "water",
            "lava",
            "short_grass",
            "tall_grass",
            "fern",
            "large_fern",
            "seagrass",
            "fire",
            "snow",
            "vine",
            "glow_lichen",
            "sculk_vein",
            "warped_roots",
            "nether_sprouts",
        ] {
            assert!(is_replaceable(name), "{name} should be replaceable");
        }
    }

    #[test]
    fn objects_and_supports_are_not_replaceable() {
        // Everything a falling block breaks on / rests on: full cubes, and non-full
        // objects — torches, saplings, signs, flowers, crops, plates, buttons,
        // rails, redstone. None is replaceable, so each breaks a faller and supports
        // a placement.
        for name in [
            "stone",
            "oak_slab",
            "oak_fence",
            "torch",
            "redstone_wire",
            "oak_sign",
            "oak_wall_sign",
            "oak_sapling",
            "dandelion",
            "poppy",
            "wheat",
            "stone_pressure_plate",
            "oak_button",
            "lever",
            "rail",
            "dead_bush",
            "sugar_cane",
        ] {
            assert!(!is_replaceable(name), "{name} should not be replaceable");
        }
    }

    #[test]
    fn non_full_solids_are_partial_supports() {
        // A falling block lands on these but breaks instead of settling atop (their
        // collision top is below a full cube). Slabs/beds via suffix.
        for name in [
            "soul_sand",
            "mud",
            "dirt_path",
            "farmland",
            "cactus",
            "honey_block",
            "pointed_dripstone",
            "amethyst_cluster",
            "chest",
            "hopper",
            "cauldron",
            "enchanting_table",
            "stonecutter",
            "daylight_detector",
            "cake",
            "campfire",
            "red_bed",
            "white_bed",
        ] {
            assert!(
                is_partial_solid_support(name),
                "{name} should be a partial support"
            );
        }
    }

    #[test]
    fn full_cubes_are_not_partial_supports() {
        // A faller settles on top of these full cubes (the resting cell above is air).
        // Slabs are resolved by state at the call site, not this name set.
        for name in [
            "stone",
            "dirt",
            "grass_block",
            "sand",
            "slime_block",
            "oak_planks",
            "oak_slab",
        ] {
            assert!(
                !is_partial_solid_support(name),
                "{name} should not be a partial support"
            );
        }
    }
}
