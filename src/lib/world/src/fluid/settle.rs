//! Finding fluid cells that need to settle after a chunk is generated or loaded.
//!
//! Terrain generation places fluids (currently water, flooded up to sea level) but never runs the
//! fluid simulation, so wherever carving leaves a fluid body open to air — a cave breaching an
//! ocean, or a spring perched on a ledge — the fluid sits in a non-equilibrium "hanging" state until
//! something ticks it. [`fluid_frontier_cells`] finds exactly those edge cells so the caller can
//! schedule a one-off fluid tick for each when the chunk becomes active near a player, mirroring
//! vanilla's settle-on-load behaviour without ticking the (usually vast, already-settled) fluid
//! interior.

use crate::block_state_id::BlockStateId;
use crate::chunk::Chunk;
use crate::fluid::fluid_state;
use crate::pos::{BlockPos, ChunkBlockPos, ChunkPos};
use ferrumc_macros::block;

/// An empty cell a fluid could flow into (air or void air). Same-fluid and solid cells are
/// deliberately not counted: a fully submerged interior cell is already in equilibrium and must not
/// be scheduled, or oceans would wake thousands of no-op ticks every time a chunk loads.
fn is_open(b: BlockStateId) -> bool {
    b == block!("air") || b == block!("void_air")
}

/// Whether the fluid cell at in-chunk `(x, y, z)` has an *in-chunk* open neighbour below or to the
/// side. Up is never checked — fluid does not flow upward. Cross-chunk neighbours are skipped (see
/// [`fluid_frontier_cells`]).
fn borders_open(chunk: &Chunk, x: u8, y: i16, z: u8, min_y: i16) -> bool {
    // Down-flow: the cell directly below, as long as it is inside this chunk.
    if y > min_y && is_open(chunk.get_block(ChunkBlockPos::new(x, y - 1, z))) {
        return true;
    }
    // Sideways spread: the four horizontal neighbours that lie within this chunk.
    for (dx, dz) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
        let nx = i32::from(x) + dx;
        let nz = i32::from(z) + dz;
        if !(0..16).contains(&nx) || !(0..16).contains(&nz) {
            continue; // cross-chunk: caught when the neighbouring chunk is settled
        }
        if is_open(chunk.get_block(ChunkBlockPos::new(nx as u8, y, nz as u8))) {
            return true;
        }
    }
    false
}

/// Returns the world positions of fluid cells in `chunk` that border open space they could flow into
/// — a fluid block with an in-chunk air neighbour below or to the side (never above; fluid does not
/// flow up).
///
/// Cross-chunk neighbours are intentionally ignored: a frontier on a chunk edge is caught when the
/// neighbouring chunk is itself settled, which keeps this a pure single-chunk scan that needs no
/// world access and never triggers neighbour generation. Sections that are entirely air hold no fluid
/// and are skipped, so the scan stays cheap for the mostly empty upper world.
pub fn fluid_frontier_cells(chunk: &Chunk, chunk_pos: ChunkPos) -> Vec<BlockPos> {
    let dims = chunk.dimensions();
    let min_y = dims.min_y;
    let base_x = chunk_pos.x() * 16;
    let base_z = chunk_pos.z() * 16;

    let mut out = Vec::new();
    for (si, section) in chunk.sections.iter().enumerate() {
        // An all-air section contains no fluid, so it cannot hold any frontier cell.
        if section.block_count() == 0 {
            continue;
        }
        let section_base_y = min_y + (si as i16) * 16;
        for ly in 0..16i16 {
            let y = section_base_y + ly;
            for x in 0..16u8 {
                for z in 0..16u8 {
                    let here = chunk.get_block(ChunkBlockPos::new(x, y, z));
                    if fluid_state(here).is_none() {
                        continue;
                    }
                    if borders_open(chunk, x, y, z, min_y) {
                        out.push(BlockPos::of(
                            base_x + i32::from(x),
                            i32::from(y),
                            base_z + i32::from(z),
                        ));
                    }
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fluid::{fluid_block, FluidKind};

    /// A water body with one column carved open to air at its side yields exactly the water cells on
    /// that exposed face as frontier cells; the submerged interior and the flat top are not flagged.
    #[test]
    fn finds_only_the_exposed_face() {
        let mut chunk = Chunk::new_empty();
        // Solid stone shell from y=60..=63 across the chunk, then fill a 4x4x4 water pocket inside
        // it, fully enclosed (every neighbour solid or water) — an equilibrium interior.
        for x in 0..16u8 {
            for z in 0..16u8 {
                for y in 60..=66i16 {
                    chunk.set_block(ChunkBlockPos::new(x, y, z), block!("stone"));
                }
            }
        }
        let water = fluid_block(FluidKind::Water, 0);
        for x in 4..8u8 {
            for z in 4..8u8 {
                for y in 61..=64i16 {
                    chunk.set_block(ChunkBlockPos::new(x, y, z), water);
                }
            }
        }

        // Fully enclosed: no frontier yet.
        assert!(
            fluid_frontier_cells(&chunk, ChunkPos::new(0, 0)).is_empty(),
            "enclosed water must produce no frontier cells"
        );

        // Carve a single air cell beside the pocket at (8, 62, 5) — the water at (7, 62, 5) now has
        // an open horizontal neighbour and should be the only frontier cell.
        chunk.set_block(ChunkBlockPos::new(8, 62, 5), block!("air"));
        let frontier = fluid_frontier_cells(&chunk, ChunkPos::new(0, 0));
        assert_eq!(
            frontier,
            vec![BlockPos::of(7, 62, 5)],
            "only the water cell facing the carved opening should be a frontier, got {frontier:?}"
        );
    }

    /// A water column standing on air (a perched spring) flags the bottom cell for down-flow.
    #[test]
    fn perched_water_flags_downflow() {
        let mut chunk = Chunk::new_empty();
        // A single water block at (8, 70, 8) with air all around and below.
        chunk.set_block(
            ChunkBlockPos::new(8, 70, 8),
            fluid_block(FluidKind::Water, 0),
        );
        let frontier = fluid_frontier_cells(&chunk, ChunkPos::new(2, -3));
        // World coords: base (32, -48); the cell at world (40, 70, -40).
        assert_eq!(frontier, vec![BlockPos::of(40, 70, -40)]);
    }
}
