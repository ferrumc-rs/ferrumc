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

/// The kind of tree to place. Kept as an enum so additional shapes can be added without changing
/// the [`Tree`] plumbing or the biome `tree_at` interface.
pub(crate) enum TreeKind {
    /// Standard rounded oak. See [`place_oak_tree`].
    Oak,
    /// Birch: the same shape as oak but with birch logs and leaves.
    Birch,
    /// Conical spruce/taiga tree with a stepped canopy and a pointed tip. See [`place_spruce_tree`].
    Spruce,
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
        TreeKind::Oak => {
            let log = block!("oak_log", { axis: "y" });
            let leaves =
                block!("oak_leaves", { distance: 1, persistent: false, waterlogged: false });
            place_oak_tree(
                chunk,
                cx,
                cz,
                tree.surface_y,
                tree.trunk_height,
                log,
                leaves,
            );
        }
        TreeKind::Birch => {
            let log = block!("birch_log", { axis: "y" });
            let leaves =
                block!("birch_leaves", { distance: 1, persistent: false, waterlogged: false });
            place_oak_tree(
                chunk,
                cx,
                cz,
                tree.surface_y,
                tree.trunk_height,
                log,
                leaves,
            );
        }
        TreeKind::Spruce => place_spruce_tree(chunk, cx, cz, tree.surface_y, tree.trunk_height),
    }
}

/// Places a standard rounded tree (oak/birch shape) with the trunk base at `surface_y + 1`. The
/// `log` and `leaves` block states select the wood type.
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
fn place_oak_tree(
    chunk: &mut Chunk,
    cx: i32,
    cz: i32,
    surface_y: i16,
    trunk_height: u8,
    log: BlockStateId,
    leaves: BlockStateId,
) {
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

/// Places a conical spruce tree with the trunk base at `surface_y + 1`.
///
/// * `trunk_height` — number of log blocks (6–8 is typical); the canopy is taller and narrower than
///   the oak shape, tapering from a 5×5 base ring up to a single-block tip one block above the top
///   log.
///
/// The canopy is built as alternating wide (radius 2) and narrow (radius 1) rings climbing the
/// trunk, so it reads as the stepped silhouette of a conifer. As with the oak shape, the trunk is
/// only written when in bounds and leaves are placed only into air.
fn place_spruce_tree(chunk: &mut Chunk, cx: i32, cz: i32, surface_y: i16, trunk_height: u8) {
    let log = block!("spruce_log", { axis: "y" });
    let leaves = block!("spruce_leaves", { distance: 1, persistent: false, waterlogged: false });

    let top_y = surface_y + i16::from(trunk_height);

    // Trunk.
    if (0..16).contains(&cx) && (0..16).contains(&cz) {
        for dy in 1..=i16::from(trunk_height) {
            chunk.set_block(ChunkBlockPos::new(cx as u8, surface_y + dy, cz as u8), log);
        }
    }

    // Stepped canopy: start two blocks below the top log and climb, alternating a wide ring and a
    // narrow ring so the silhouette tapers. The canopy spans the upper portion of the trunk.
    let canopy_height = i16::from(trunk_height).min(6);
    for step in 0..canopy_height {
        let y = top_y - 1 - step;
        // Wide rings on even steps from the bottom, narrowing toward the top.
        let radius = if step >= canopy_height - 2 {
            0 // top: single column of leaves above the trunk handled by the tip below
        } else if (canopy_height - 1 - step) % 2 == 0 {
            2
        } else {
            1
        };
        if radius == 0 {
            continue;
        }
        place_leaf_ring(chunk, cx, cz, y, radius, leaves);
    }

    // Narrow cap and pointed tip.
    place_leaf_ring(chunk, cx, cz, top_y, 1, leaves);
    if (0..16).contains(&cx) && (0..16).contains(&cz) {
        let tip = ChunkBlockPos::new(cx as u8, top_y + 1, cz as u8);
        if match_block!("air", chunk.get_block(tip)) {
            chunk.set_block(tip, leaves);
        }
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
