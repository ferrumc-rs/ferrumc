//! Tree shape functions.
//!
//! Trees are placed with cross-chunk overscan: a tree whose trunk falls near a chunk edge drops
//! canopy blocks into the neighbouring chunk(s). Rather than writing into other chunks (which would
//! require shared, synchronised state and break per-chunk parallelism), every chunk independently
//! places *only the blocks of each nearby tree that fall within its own `0..16` bounds*. Tree
//! placement is a pure function of the world seed and the trunk's global column, so neighbouring
//! chunks resolve the exact same trees and their portions tile seamlessly.
//!
//! Accordingly, the placement functions take *signed* local coordinates (`cx`, `cz` may be negative
//! or `>= 16` when the trunk lives in an adjacent chunk) and silently clip any block that falls
//! outside the current chunk.

use ferrumc_macros::{block, match_block};
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::ChunkBlockPos;

/// Maximum horizontal distance (in blocks) from a trunk to the outermost canopy block. Chunk
/// generation must overscan its neighbours by this many columns so cross-chunk canopies are placed.
pub(crate) const MAX_CANOPY_RADIUS: i32 = 2;

/// The kind of tree to place. Currently only oak; kept as an enum so additional shapes can be added
/// without changing the [`Tree`] plumbing or the biome `tree_at` interface.
pub(crate) enum TreeKind {
    Oak,
}

/// A fully-resolved tree to place at a known global column: its shape, the surface height its trunk
/// base sits on, and its trunk height. Produced by a biome's `tree_at` and consumed by
/// [`place_tree`]; it carries no coordinates so the same value can be placed (clipped) into any
/// chunk the canopy overlaps.
pub(crate) struct Tree {
    pub kind: TreeKind,
    pub surface_y: i16,
    pub trunk_height: u8,
}

/// Places `tree`'s blocks into `chunk`, where `(cx, cz)` is the trunk column expressed in this
/// chunk's local space (may be outside `0..16` for a trunk in a neighbouring chunk). Only blocks
/// inside the chunk are written; everything else is clipped.
pub(crate) fn place_tree(chunk: &mut Chunk, cx: i32, cz: i32, tree: &Tree) {
    match tree.kind {
        TreeKind::Oak => place_oak_tree(chunk, cx, cz, tree.surface_y, tree.trunk_height),
    }
}

/// Places a standard oak tree with the trunk base at `surface_y + 1`.
///
/// * `trunk_height` — number of log blocks (4–6 is typical).
///
/// Leaf canopy layout (relative to the top log at `surface_y + trunk_height`):
/// * dy −2, −1 : 5×5 ring with all four diagonal corners removed.
/// * dy  0, +1 : 3×3 diamond (axis-aligned cross, no diagonal corners).
///
/// The trunk is a single column, so it is only written when it lies inside this chunk; leaves are
/// placed only into air, leaving existing blocks (logs from an overlapping tree, stone peeking
/// through, etc.) untouched.
fn place_oak_tree(chunk: &mut Chunk, cx: i32, cz: i32, surface_y: i16, trunk_height: u8) {
    let log = block!("oak_log", { axis: "y" });
    let leaves = block!("oak_leaves", { distance: 1, persistent: false, waterlogged: false });

    let top_y = surface_y + i16::from(trunk_height);

    // Trunk: surface + 1 to surface + trunk_height (inclusive). Only this chunk's own trunks land
    // in bounds; a neighbour tree's trunk column is clipped here and placed by its home chunk.
    if (0..16).contains(&cx) && (0..16).contains(&cz) {
        for dy in 1..=i16::from(trunk_height) {
            chunk.set_block(ChunkBlockPos::new(cx as u8, surface_y + dy, cz as u8), log);
        }
    }

    // Two wide leaf layers (5×5 minus diagonal corners).
    for dy in [-2i16, -1] {
        place_leaf_ring(chunk, cx, cz, top_y + dy, 2, leaves);
    }

    // Two narrow leaf layers (3×3 minus diagonal corners = diamond cross).
    for dy in [0i16, 1] {
        place_leaf_ring(chunk, cx, cz, top_y + dy, 1, leaves);
    }
}

/// Places a square leaf ring of the given `radius` centered on `(cx, cz)` at height `y`,
/// with all four diagonal corners (|dx| == radius && |dz| == radius) removed.
/// Blocks outside chunk bounds (0..16) or already non-air are silently skipped.
fn place_leaf_ring(chunk: &mut Chunk, cx: i32, cz: i32, y: i16, radius: i32, leaf: BlockStateId) {
    let r = radius;
    for dx in -r..=r {
        for dz in -r..=r {
            if dx.abs() == r && dz.abs() == r {
                continue; // diagonal corner — omit for a rounded shape
            }

            let lx = cx + dx;
            let lz = cz + dz;

            // Clip leaves at chunk boundaries; the neighbouring chunk places its own share.
            if !(0..16).contains(&lx) || !(0..16).contains(&lz) {
                continue;
            }

            let pos = ChunkBlockPos::new(lx as u8, y, lz as u8);
            if match_block!("air", chunk.get_block(pos)) {
                chunk.set_block(pos, leaf);
            }
        }
    }
}
