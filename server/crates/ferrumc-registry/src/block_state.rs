//! Fixed block-*state* ids for the minimal flat-world block set.
//!
//! A block-state id is the integer written into a chunk section's block palette
//! on the wire. Unlike biome and dimension-type ids — which are server-assigned
//! indices into the dynamic registries sent during the configuration phase —
//! block-state ids are **fixed protocol constants**: they must match the values
//! baked into the vanilla 1.21.8 client jar, or the client renders the wrong
//! blocks.
//!
//! The values here are the *default* states of each block (the state a freshly
//! placed block takes), taken from the vendored `blocks.json` snapshot. A
//! `#[cfg(test)]` drift guard in the crate root re-parses that snapshot and
//! asserts these constants still match, so a re-pin to a newer data version
//! cannot silently desync them.
//!
//! The flat-world generator (in `ferrumc-world`) layers these as:
//! bedrock floor, [`STONE`] fill, [`DIRT`] subsurface, [`GRASS_BLOCK`] surface,
//! and [`AIR`] above.

/// Block-state id for `minecraft:air`.
pub const AIR: u32 = 0;

/// Block-state id for `minecraft:stone`.
pub const STONE: u32 = 1;

/// Block-state id for the default state of `minecraft:grass_block`.
///
/// `grass_block` has a single `snowy` boolean property spanning state ids 8
/// (`snowy=true`) and 9 (`snowy=false`); the default is `snowy=false` → `9`.
pub const GRASS_BLOCK: u32 = 9;

/// Block-state id for `minecraft:dirt` (block id 9, single state).
pub const DIRT: u32 = 10;

/// Block-state id for `minecraft:bedrock` (block id 34, single state).
pub const BEDROCK: u32 = 85;

use std::collections::BTreeMap;

/// The kind of a block-state [`Property`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyType {
    /// An enumerated (or integer) property with an explicit ordered value list.
    Enum,
    /// A boolean property; its values are `["true", "false"]` (index `0` = `true`),
    /// matching the vanilla state-id encoding order.
    Bool,
}

/// One property of a block, in the order it contributes to the linear state-id
/// encoding (the first property is the most-significant digit; the last varies
/// fastest).
#[derive(Debug, Clone, Copy)]
pub struct Property {
    /// The property name (for example `"axis"`, `"facing"`, `"waterlogged"`).
    pub name: &'static str,
    /// Whether the property is a boolean or an explicitly enumerated value list.
    pub typ: PropertyType,
    /// The number of distinct values (equal to `values.len()`).
    pub cardinality: usize,
    /// The ordered value strings; a value's index in this slice is its encoding
    /// digit.
    pub values: &'static [&'static str],
}

/// Static, build-time-generated metadata for one block and its dense state range.
///
/// Produced from the vendored `data/blocks.json` snapshot in `build.rs`; the
/// runtime only reads these `'static` tables (no JSON parsing, no allocation).
#[derive(Debug, Clone, Copy)]
pub struct BlockMetadata {
    /// The bare resource name (no `minecraft:` namespace), e.g. `"oak_log"`.
    pub name: &'static str,
    /// The block's protocol id.
    pub id: u16,
    /// The default state id — the state a freshly placed block takes.
    pub default_state: u32,
    /// The lowest state id in this block's dense range.
    pub min_state: u32,
    /// The highest state id in this block's dense range.
    pub max_state: u32,
    /// Whether the block is a full solid cube (`boundingBox == "block"`), used by
    /// the placement layer to decide fence connectivity to non-fence neighbours.
    pub is_solid_cube: bool,
    /// Collision-box top height (max Y, in blocks) per state. Length `1` when the
    /// height is uniform across states (the common case), otherwise one entry per
    /// state in state-id order (slabs, snow layers, dripstone differ across states).
    /// See [`collision_top_y`] for the per-state lookup.
    pub collision_top: &'static [f32],
    /// The block's properties, in state-id encoding order.
    pub properties: &'static [Property],
}

// Pulls in `BLOCK_COUNT`, `BLOCKS`, `BLOCKS_BY_NAME`, and `BLOCKS_BY_MIN_STATE`,
// generated from `data/blocks.json` at build time (see `build.rs`).
include!(concat!(env!("OUT_DIR"), "/blocks_generated.rs"));

/// Strips the optional canonical `minecraft:` namespace from a resource name.
fn bare_name(name: &str) -> &str {
    name.strip_prefix("minecraft:").unwrap_or(name)
}

/// Returns the static [`BlockMetadata`] for a block resource name (namespaced or
/// bare), or `None` if the block is not in the pinned 1.21.8 registry.
///
/// The lookup is a binary search over a name-sorted static table, so it is
/// allocation-free and deterministic.
#[must_use]
pub fn block_metadata(name: &str) -> Option<&'static BlockMetadata> {
    let bare = bare_name(name);
    let pos = BLOCKS_BY_NAME
        .binary_search_by(|&(n, _)| n.cmp(bare))
        .ok()?;
    Some(&BLOCKS[BLOCKS_BY_NAME[pos].1])
}

/// Returns the default state id for a block resource name (namespaced or bare),
/// or `None` if the block is unknown.
///
/// Unlike [`crate::default_block_state_id`] (which pins only the flat-world set),
/// this resolves *every* block in the registry.
#[must_use]
pub fn block_default_state(name: &str) -> Option<u32> {
    block_metadata(name).map(|m| m.default_state)
}

/// Reverse lookup: returns the bare resource name of the block that owns
/// `state_id`, or `None` if no block's dense range contains it.
///
/// Binary-searches the min-state-sorted, disjoint range table. Used by the
/// placement neighbour query to resolve a neighbour's state id back to a block
/// name.
#[must_use]
pub fn state_id_to_block_name(state_id: u32) -> Option<&'static str> {
    // Index of the first range whose min exceeds state_id; the candidate is the
    // one just before it (the largest min <= state_id). Ranges are disjoint, so
    // at most one range can contain state_id.
    let after = BLOCKS_BY_MIN_STATE.partition_point(|&(min, _, _)| min <= state_id);
    let (_, max, idx) = *BLOCKS_BY_MIN_STATE.get(after.checked_sub(1)?)?;
    (state_id <= max).then(|| BLOCKS[idx].name)
}

/// Returns the collision-box top height (max Y, in blocks) of the block state
/// `state_id`, or `None` when no block owns that id.
///
/// The value classifies how a falling block interacts with the state as a *support*:
/// `0.0` is no collision (a faller passes through, landing on the block below);
/// `>= 1.0` is a full-height top (a faller settles on top of it); `0.0 < h < 1.0` is
/// a non-full support (a slab, soul sand, dripstone tip, …) that a faller *lands on*
/// but breaks against instead of settling, matching vanilla. Resolved per state, so
/// a bottom slab (`0.5`) and a double slab (`1.0`) differ.
///
/// Binary-searches the same disjoint range table as [`state_id_to_block_name`], then
/// indexes the owning block's per-state height slice (a uniform block stores one
/// value shared by every state).
#[must_use]
pub fn collision_top_y(state_id: u32) -> Option<f32> {
    let after = BLOCKS_BY_MIN_STATE.partition_point(|&(min, _, _)| min <= state_id);
    let (min, max, idx) = *BLOCKS_BY_MIN_STATE.get(after.checked_sub(1)?)?;
    if state_id > max {
        return None;
    }
    let tops = BLOCKS[idx].collision_top;
    if tops.len() == 1 {
        tops.first().copied()
    } else {
        tops.get((state_id - min) as usize).copied()
    }
}

/// Returns the place value (multiplier) of property `pi` in the block's linear
/// state encoding: the product of the cardinalities of all *later* properties.
fn multiplier(meta: &BlockMetadata, pi: usize) -> u32 {
    meta.properties[pi + 1..]
        .iter()
        .map(|p| p.cardinality as u32)
        .product()
}

/// Resolves a block name plus a property override map to a concrete state id.
///
/// The computation starts from the block's *default* state and overrides only the
/// named properties, **inheriting** every unspecified property (waterlogged, lit,
/// shape, …) from the default state. This is deliberately more ergonomic than a
/// strict "all properties required" contract: placement rules only ever set the
/// subset of properties they compute, and inherit the rest.
///
/// Returns `None` for an unknown block, an unknown property name, or a value that
/// is not one of that property's values (so a caller's typo fails loudly rather
/// than silently landing the wrong block).
///
/// # Examples
///
/// ```
/// use std::collections::BTreeMap;
/// use ferrumc_registry::block_state::compute_state_id;
///
/// // `axis=y` is the oak-log default, so the empty/default map gives 137.
/// assert_eq!(compute_state_id("oak_log", &BTreeMap::new()), Some(137));
///
/// let mut props = BTreeMap::new();
/// props.insert("axis", "x");
/// assert_eq!(compute_state_id("oak_log", &props), Some(136));
/// ```
#[must_use]
pub fn compute_state_id(block_name: &str, properties: &BTreeMap<&str, &str>) -> Option<u32> {
    let meta = block_metadata(block_name)?;
    let mut state = meta.default_state;
    for (&prop_name, &value) in properties {
        let pi = meta.properties.iter().position(|p| p.name == prop_name)?;
        let new_index = meta.properties[pi]
            .values
            .iter()
            .position(|v| *v == value)? as u32;
        let mult = multiplier(meta, pi);
        let card = meta.properties[pi].cardinality as u32;
        // The default state's digit for this property. The linear encoding is
        // separable, so replacing one property's digit never disturbs another's.
        let old_index = ((meta.default_state - meta.min_state) / mult) % card;
        state = state - old_index * mult + new_index * mult;
    }
    Some(state)
}

#[cfg(test)]
mod catalog_tests {
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use super::{
        block_default_state, block_metadata, collision_top_y, compute_state_id,
        state_id_to_block_name, PropertyType, BLOCK_COUNT,
    };

    /// The vendored snapshot the generated tables are built from.
    #[derive(serde::Deserialize)]
    struct RawBlock {
        name: String,
        #[serde(rename = "defaultState")]
        default_state: u32,
        #[serde(rename = "minStateId")]
        min_state_id: u32,
        #[serde(rename = "maxStateId")]
        max_state_id: u32,
        states: Vec<RawState>,
        #[serde(rename = "boundingBox")]
        bounding_box: String,
    }

    #[derive(serde::Deserialize)]
    struct RawState {
        name: String,
        #[serde(rename = "type")]
        typ: String,
        num_values: usize,
        #[serde(default)]
        values: Option<Vec<String>>,
    }

    fn raw_blocks() -> Vec<RawBlock> {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/blocks.json");
        let raw = std::fs::read_to_string(path).expect("vendored data/blocks.json must exist");
        serde_json::from_str(&raw).expect("blocks.json parses")
    }

    /// The expected (bool-normalised) value list for a raw state.
    fn expected_values(state: &RawState) -> Vec<String> {
        if state.typ == "bool" {
            vec!["true".to_string(), "false".to_string()]
        } else {
            state.values.clone().expect("enum/int carries values")
        }
    }

    #[test]
    fn block_count_matches_snapshot() {
        assert_eq!(BLOCK_COUNT, raw_blocks().len());
        assert_eq!(BLOCK_COUNT, 1105);
    }

    /// Drift guard: every block's generated metadata (range, default, properties
    /// and their ordered values, solid-cube flag) matches the vendored snapshot,
    /// and the default state round-trips through `compute_state_id`.
    #[test]
    fn generated_metadata_matches_snapshot_and_round_trips() {
        for raw in raw_blocks() {
            let meta = block_metadata(&raw.name)
                .unwrap_or_else(|| panic!("{} missing from generated tables", raw.name));
            assert_eq!(meta.name, raw.name);
            assert_eq!(
                meta.default_state, raw.default_state,
                "{} default",
                raw.name
            );
            assert_eq!(meta.min_state, raw.min_state_id, "{} min", raw.name);
            assert_eq!(meta.max_state, raw.max_state_id, "{} max", raw.name);
            assert_eq!(
                meta.is_solid_cube,
                raw.bounding_box == "block",
                "{} solidity",
                raw.name
            );
            assert_eq!(
                meta.properties.len(),
                raw.states.len(),
                "{} property count",
                raw.name
            );

            for (gen_prop, raw_state) in meta.properties.iter().zip(&raw.states) {
                assert_eq!(gen_prop.name, raw_state.name, "{} prop name", raw.name);
                let want_bool = raw_state.typ == "bool";
                assert_eq!(
                    gen_prop.typ == PropertyType::Bool,
                    want_bool,
                    "{} prop {} type",
                    raw.name,
                    raw_state.name
                );
                let want_values = expected_values(raw_state);
                assert_eq!(
                    gen_prop.cardinality, raw_state.num_values,
                    "{} prop {} cardinality",
                    raw.name, raw_state.name
                );
                assert_eq!(
                    gen_prop.values,
                    want_values.iter().map(String::as_str).collect::<Vec<_>>(),
                    "{} prop {} values/order",
                    raw.name,
                    raw_state.name
                );
            }

            // Empty override map inherits everything -> the default state.
            assert_eq!(
                compute_state_id(&raw.name, &BTreeMap::new()),
                Some(raw.default_state),
                "{} default round-trip",
                raw.name
            );
            // Every default-state digit set explicitly still yields the default.
            let span = meta.max_state - meta.min_state;
            let mut rem = raw.default_state - raw.min_state_id;
            let mut props: BTreeMap<&str, &str> = BTreeMap::new();
            let mut digits: Vec<usize> = Vec::new();
            for (i, prop) in meta.properties.iter().enumerate() {
                let later: u32 = meta.properties[i + 1..]
                    .iter()
                    .map(|p| p.cardinality as u32)
                    .product();
                let digit = (rem / later) as usize % prop.cardinality;
                digits.push(digit);
                rem %= later;
            }
            for (prop, &digit) in meta.properties.iter().zip(&digits) {
                props.insert(prop.name, prop.values[digit]);
            }
            assert_eq!(
                compute_state_id(&raw.name, &props),
                Some(raw.default_state),
                "{} explicit-default round-trip (span {span})",
                raw.name
            );

            // The reverse lookup resolves min, default, and max back to this block.
            assert_eq!(state_id_to_block_name(meta.min_state), Some(meta.name));
            assert_eq!(state_id_to_block_name(meta.default_state), Some(meta.name));
            assert_eq!(state_id_to_block_name(meta.max_state), Some(meta.name));
        }
    }

    /// Drift guard: every generated `ids::*` constant equals the value the
    /// runtime lookups return for its block. The `ids` consts and this table are
    /// emitted from the same parsed `blocks.json` as `block_default_state` /
    /// `compute_state_id`, so a curated const can never silently rot away from
    /// the lookup path (or a re-vendor that shifts an id fails here).
    #[test]
    fn id_constants_match_runtime_lookups() {
        assert!(
            !super::IDS_DRIFT_TABLE.is_empty(),
            "curated ids table must not be empty"
        );
        for (name, value) in super::IDS_DRIFT_TABLE {
            assert_eq!(
                block_default_state(name),
                Some(value),
                "ids const for {name} disagrees with block_default_state"
            );
            assert_eq!(
                compute_state_id(name, &BTreeMap::new()),
                Some(value),
                "ids const for {name} disagrees with compute_state_id"
            );
        }
    }

    #[test]
    fn block_default_state_resolves_namespaced_and_bare() {
        assert_eq!(block_default_state("minecraft:oak_log"), Some(137));
        assert_eq!(block_default_state("oak_log"), Some(137));
        assert_eq!(block_default_state("definitely_not_a_block"), None);
    }

    /// Logs rotate via the `axis` enum (`x`/`y`/`z`).
    #[test]
    fn log_axis_state_ids() {
        let id = |axis: &str| {
            let mut p = BTreeMap::new();
            p.insert("axis", axis);
            compute_state_id("oak_log", &p)
        };
        assert_eq!(id("x"), Some(136));
        assert_eq!(id("y"), Some(137)); // default
        assert_eq!(id("z"), Some(138));
    }

    /// Slabs select `type` (`top`/`bottom`/`double`); `waterlogged` inherits false.
    #[test]
    fn slab_type_state_ids() {
        let id = |ty: &str| {
            let mut p = BTreeMap::new();
            p.insert("type", ty);
            compute_state_id("oak_slab", &p)
        };
        assert_eq!(id("top"), Some(12052));
        assert_eq!(id("bottom"), Some(12054)); // default
        assert_eq!(compute_state_id("oak_slab", &BTreeMap::new()), Some(12054));
    }

    /// Stairs select `facing` + `half`; `shape`/`waterlogged` inherit the default.
    #[test]
    fn stair_facing_half_state_ids() {
        let id = |facing: &str, half: &str| {
            let mut p = BTreeMap::new();
            p.insert("facing", facing);
            p.insert("half", half);
            compute_state_id("oak_stairs", &p)
        };
        assert_eq!(id("north", "bottom"), Some(2949)); // default
        assert_eq!(id("west", "top"), Some(2979));
        assert_eq!(id("south", "bottom"), Some(2969));
    }

    /// Wall torches face one cardinal direction.
    #[test]
    fn wall_torch_facing_state_ids() {
        let id = |facing: &str| {
            let mut p = BTreeMap::new();
            p.insert("facing", facing);
            compute_state_id("wall_torch", &p)
        };
        assert_eq!(id("north"), Some(2402));
        assert_eq!(id("south"), Some(2403));
        assert_eq!(id("west"), Some(2404));
        assert_eq!(id("east"), Some(2405));
        // Floor torch is a single state.
        assert_eq!(block_default_state("torch"), Some(2401));
    }

    /// Fence connectivity is alphabetical booleans; a connection sets that
    /// direction `true` (digit 0), subtracting from the all-false default (6027).
    #[test]
    fn fence_connectivity_state_ids() {
        assert_eq!(compute_state_id("oak_fence", &BTreeMap::new()), Some(6027));
        let mut north = BTreeMap::new();
        north.insert("north", "true");
        assert_eq!(compute_state_id("oak_fence", &north), Some(6019));
        let mut north_east = BTreeMap::new();
        north_east.insert("north", "true");
        north_east.insert("east", "true");
        assert_eq!(compute_state_id("oak_fence", &north_east), Some(6003));
        let mut all_four = BTreeMap::new();
        for d in ["north", "south", "east", "west"] {
            all_four.insert(d, "true");
        }
        assert_eq!(compute_state_id("oak_fence", &all_four), Some(5998));
    }

    /// Horizontal-facing blocks set `facing` and inherit `lit` from the default.
    #[test]
    fn furnace_facing_inherits_lit() {
        let mut p = BTreeMap::new();
        p.insert("facing", "north");
        assert_eq!(compute_state_id("furnace", &p), Some(4359)); // default (lit=false)
    }

    /// Simple cubes have a single state.
    #[test]
    fn simple_cube_state_ids() {
        assert_eq!(compute_state_id("stone", &BTreeMap::new()), Some(1));
        assert_eq!(compute_state_id("dirt", &BTreeMap::new()), Some(10));
    }

    #[test]
    fn compute_state_id_rejects_unknown_block_property_or_value() {
        assert_eq!(compute_state_id("not_a_block", &BTreeMap::new()), None);
        let mut bad_prop = BTreeMap::new();
        bad_prop.insert("nonsense", "x");
        assert_eq!(compute_state_id("oak_log", &bad_prop), None);
        let mut bad_value = BTreeMap::new();
        bad_value.insert("axis", "w");
        assert_eq!(compute_state_id("oak_log", &bad_value), None);
    }

    #[test]
    fn state_id_to_block_name_spot_checks() {
        assert_eq!(state_id_to_block_name(0), Some("air"));
        assert_eq!(state_id_to_block_name(137), Some("oak_log"));
        assert_eq!(state_id_to_block_name(2403), Some("wall_torch"));
        assert_eq!(state_id_to_block_name(6003), Some("oak_fence"));
        // Past the end of the registry there is no owning block.
        assert_eq!(state_id_to_block_name(u32::MAX), None);
    }

    #[test]
    fn collision_top_y_classifies_supports() {
        // Full cubes top at 1.0; open blocks at 0.0; partials in between.
        let full = |name: &str| collision_top_y(block_default_state(name).unwrap());
        assert_eq!(full("stone"), Some(1.0));
        assert_eq!(full("torch"), Some(0.0));
        assert_eq!(full("soul_sand"), Some(0.875));
        assert_eq!(full("farmland"), Some(0.9375));
        assert_eq!(full("repeater"), Some(0.125));

        // Per-state: a bottom slab is half height, a double slab a full cube.
        let bottom = compute_state_id(
            "oak_slab",
            &BTreeMap::from([("type", "bottom"), ("waterlogged", "false")]),
        )
        .unwrap();
        let double = compute_state_id(
            "oak_slab",
            &BTreeMap::from([("type", "double"), ("waterlogged", "false")]),
        )
        .unwrap();
        assert_eq!(collision_top_y(bottom), Some(0.5));
        assert_eq!(collision_top_y(double), Some(1.0));

        assert_eq!(collision_top_y(u32::MAX), None);
    }
}
