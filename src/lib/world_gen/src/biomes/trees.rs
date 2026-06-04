//! Tree shape functions. Each function places a single tree into a chunk, clipping any
//! blocks that fall outside the 0..16 column bounds (Option B: boundary leaves are simply
//! omitted rather than written into neighbouring chunks).

use ferrumc_macros::{block, match_block};
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::ChunkBlockPos;

/// Places a standard oak tree with the trunk base at `surface_y + 1`.
///
/// * `trunk_height` — number of log blocks (4–6 is typical).
///
/// Leaf canopy layout (relative to the top log at `surface_y + trunk_height`):
/// * dy −2, −1 : 5×5 ring with all four diagonal corners removed.
/// * dy  0, +1 : 3×3 diamond (axis-aligned cross, no diagonal corners).
///
/// Leaves are only placed into air; existing blocks (logs from an overlapping tree, stone
/// peeking through, etc.) are left untouched.
pub(crate) fn place_oak_tree(chunk: &mut Chunk, x: u8, z: u8, surface_y: i16, trunk_height: u8) {
    let log = block!("oak_log", { axis: "y" });
    let leaves = block!("oak_leaves", { distance: 1, persistent: false, waterlogged: false });

    let top_y = surface_y + i16::from(trunk_height);

    // Trunk: surface + 1 to surface + trunk_height (inclusive).
    for dy in 1..=i16::from(trunk_height) {
        chunk.set_block(ChunkBlockPos::new(x, surface_y + dy, z), log);
    }

    // Two wide leaf layers (5×5 minus diagonal corners).
    for dy in [-2i16, -1] {
        place_leaf_ring(chunk, x, z, top_y + dy, 2, leaves);
    }

    // Two narrow leaf layers (3×3 minus diagonal corners = diamond cross).
    for dy in [0i16, 1] {
        place_leaf_ring(chunk, x, z, top_y + dy, 1, leaves);
    }
}

/// Places a square leaf ring of the given `radius` centered on `(cx, cz)` at height `y`,
/// with all four diagonal corners (|dx| == radius && |dz| == radius) removed.
/// Blocks outside chunk bounds (0..16) or already non-air are silently skipped.
fn place_leaf_ring(chunk: &mut Chunk, cx: u8, cz: u8, y: i16, radius: i8, leaf: BlockStateId) {
    let r = radius;
    for dx in -r..=r {
        for dz in -r..=r {
            if dx.abs() == r && dz.abs() == r {
                continue; // diagonal corner — omit for a rounded shape
            }

            let lx = i16::from(cx) + i16::from(dx);
            let lz = i16::from(cz) + i16::from(dz);

            // Option B: silently clip leaves at chunk boundaries.
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
