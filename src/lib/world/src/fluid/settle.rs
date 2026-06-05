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
use crate::dimension::Dimension;
use crate::fluid::spread::{fluid_neighbours, BlockView};
use crate::fluid::{compute_tick, fluid_state, is_fluid, is_solid_obstacle, FluidRules};
use crate::pos::{BlockPos, ChunkBlockPos, ChunkPos};
use ferrumc_config::server_config::FluidAlgorithm;
use ferrumc_macros::block;
use std::collections::VecDeque;

/// Block a fluid tick reads when it probes outside the chunk being settled. The kernel treats it as a
/// solid wall, so generation-time settling never reaches into neighbouring (possibly ungenerated)
/// chunks; cross-chunk flow at the seam is left to the on-load settle pass once both sides exist.
const SETTLE_BARRIER: BlockStateId = block!("stone");

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
/// world access and never triggers neighbour generation. Any section that holds no fluid (all air,
/// all stone, cave stone+air, …) is skipped via a cheap palette-only check, so the per-cell scan only
/// touches the few sections that actually contain fluid — most chunks carry none and cost almost
/// nothing.
pub fn fluid_frontier_cells(chunk: &Chunk, chunk_pos: ChunkPos) -> Vec<BlockPos> {
    let dims = chunk.dimensions();
    let min_y = dims.min_y;
    let base_x = chunk_pos.x() * 16;
    let base_z = chunk_pos.z() * 16;

    let is_fluid = |b| fluid_state(b).is_some();

    let mut out = Vec::new();
    for (si, section) in chunk.sections.iter().enumerate() {
        // Skip any section that holds no fluid at all (all air, all stone, cave stone+air, …) using a
        // palette-only check, so the per-cell scan below only runs on the few sections that actually
        // contain fluid. This is what keeps settling cheap for the overwhelming majority of chunks,
        // which carry no fluid anywhere in their solid volume.
        if !section.any_block(is_fluid) {
            continue;
        }
        let section_base_y = min_y + (si as i16) * 16;

        // Ocean-interior fast path: a section that is entirely one fluid (a deep-water layer) can
        // only have a frontier along its *bottom* face — every in-section and upward neighbour is the
        // same fluid, and fluid never flows up. So instead of scanning 4096 cells, check whether the
        // section directly below contains any open cell at all; if not, the whole layer is interior
        // and contributes nothing, and if so only its bottom row can flow down. Stacked deep-water
        // layers therefore skip almost entirely.
        if section.uniform_block().is_some_and(is_fluid) {
            let below_has_open = si > 0 && chunk.sections[si - 1].any_block(is_open);
            if !below_has_open || section_base_y <= min_y {
                continue;
            }
            let y = section_base_y; // bottom row of this section
            for x in 0..16u8 {
                for z in 0..16u8 {
                    if is_open(chunk.get_block(ChunkBlockPos::new(x, y - 1, z))) {
                        out.push(BlockPos::of(
                            base_x + i32::from(x),
                            i32::from(y),
                            base_z + i32::from(z),
                        ));
                    }
                }
            }
            continue;
        }

        // General path: a mixed section (coastline, sea floor, cave-breached water) — scan each cell.
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

/// A read-only [`BlockView`] over a single chunk: in-chunk cells read from the chunk, everything
/// outside reads back as [`SETTLE_BARRIER`] so a settle never escapes the chunk.
struct ChunkLocalView<'a> {
    chunk: &'a Chunk,
    base_x: i32,
    base_z: i32,
}

impl ChunkLocalView<'_> {
    /// Local `(x, z)` of a world position if it lies within this chunk's column footprint.
    fn local_xz(&self, pos: BlockPos) -> Option<(u8, u8)> {
        let lx = pos.pos.x - self.base_x;
        let lz = pos.pos.z - self.base_z;
        if (0..16).contains(&lx) && (0..16).contains(&lz) {
            Some((lx as u8, lz as u8))
        } else {
            None
        }
    }
}

impl BlockView for ChunkLocalView<'_> {
    fn block_at(&self, pos: BlockPos) -> BlockStateId {
        match self.local_xz(pos) {
            Some((x, z)) => self
                .chunk
                .get_block(ChunkBlockPos::new(x, pos.pos.y as i16, z)),
            None => SETTLE_BARRIER,
        }
    }
}

/// Flows the fluids inside a freshly generated `chunk` to a steady state, in isolation (chunk borders
/// act as walls), mutating the chunk in place. This is the generation-time counterpart to the on-load
/// settle: run on the chunk-generation worker thread so the chunk arrives already settled and no fluid
/// simulation is needed on the game-tick thread for it.
///
/// `max_changes` bounds the work so a pathological chunk cannot stall generation; if the budget is hit
/// the chunk is left partially settled and the on-load pass (if enabled) finishes the remainder. A
/// budget of `0` means unbounded.
///
/// Cross-chunk flow is intentionally not resolved here (no neighbour data exists at generation time);
/// the seam is handled by the on-load settle once both chunks are loaded.
pub fn settle_chunk(
    chunk: &mut Chunk,
    chunk_pos: ChunkPos,
    dim: Dimension,
    algorithm: FluidAlgorithm,
    max_changes: usize,
) {
    let base_x = chunk_pos.x() * 16;
    let base_z = chunk_pos.z() * 16;

    // Seed the work queue with the cells that are out of equilibrium (a fluid bordering open space).
    let mut queue: VecDeque<BlockPos> =
        fluid_frontier_cells(chunk, chunk_pos).into_iter().collect();
    if queue.is_empty() {
        return;
    }

    let in_chunk = |p: BlockPos| -> Option<(u8, u8)> {
        let lx = p.pos.x - base_x;
        let lz = p.pos.z - base_z;
        ((0..16).contains(&lx) && (0..16).contains(&lz)).then_some((lx as u8, lz as u8))
    };

    let mut applied = 0usize;
    while let Some(pos) = queue.pop_front() {
        if max_changes != 0 && applied >= max_changes {
            break;
        }
        let Some((x, z)) = in_chunk(pos) else {
            continue;
        };
        let here = chunk.get_block(ChunkBlockPos::new(x, pos.pos.y as i16, z));
        let Some(state) = fluid_state(here) else {
            continue;
        };
        let rules = FluidRules::for_kind(state.kind, dim);

        // Compute this cell's changes against the current chunk (read-only), then apply them.
        let changes = {
            let view = ChunkLocalView {
                chunk,
                base_x,
                base_z,
            };
            compute_tick(algorithm, pos, &view, rules)
        };

        for change in changes {
            // Never write outside the chunk (cross-chunk seam left for the on-load pass).
            let Some((cx, cz)) = in_chunk(change.pos) else {
                continue;
            };
            let bp = ChunkBlockPos::new(cx, change.pos.pos.y as i16, cz);
            let current = chunk.get_block(bp);
            if current == change.new_block {
                continue;
            }
            // Same guard as the live apply: a plain fluid flow must not eat a solid block (reactions,
            // which turn fluid into rock, are allowed).
            if is_fluid(change.new_block) && is_solid_obstacle(current) {
                continue;
            }
            chunk.set_block(bp, change.new_block);
            applied += 1;

            // Re-examine the changed cell (if it may keep evolving) and its in-chunk neighbours so the
            // adjustment ripples to a steady state, mirroring the live system's neighbour wake.
            if change.reschedule {
                queue.push_back(change.pos);
            }
            for n in fluid_neighbours(change.pos) {
                if in_chunk(n).is_some() {
                    queue.push_back(n);
                }
            }
        }
    }
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

    /// Exercises the ocean-interior fast path: a full water section resting on a solid section is
    /// interior (no frontier), while the same water section resting on air flags its entire bottom
    /// row for down-flow.
    #[test]
    fn uniform_water_section_only_flags_open_bottom() {
        let water = fluid_block(FluidKind::Water, 0);

        // Water section (y 0..=15) on a stone section (y -16..=-1): fully interior, no frontier.
        let mut on_solid = Chunk::new_empty();
        on_solid.fill_section(-1, block!("stone"));
        on_solid.fill_section(0, water);
        assert!(
            fluid_frontier_cells(&on_solid, ChunkPos::new(0, 0)).is_empty(),
            "a water section resting on solid ground is interior and has no frontier"
        );

        // Same water section but the section below is air: the whole bottom row can flow down.
        let mut on_air = Chunk::new_empty();
        on_air.fill_section(0, water);
        let frontier = fluid_frontier_cells(&on_air, ChunkPos::new(0, 0));
        assert_eq!(
            frontier.len(),
            256,
            "a water section resting on air flags its entire bottom row"
        );
        assert!(
            frontier.iter().all(|p| p.pos.y == 0),
            "all down-flow frontier cells sit on the section's bottom row (y = 0)"
        );
    }

    /// `settle_chunk` flows a hanging source to a steady state in place: a source on a floor with air
    /// around it spreads across the floor; an unbounded budget settles it fully.
    #[test]
    fn settle_chunk_flows_a_source_across_the_floor() {
        let mut chunk = Chunk::new_empty();
        // Stone floor at y=64 across the whole chunk, air above.
        for x in 0..16u8 {
            for z in 0..16u8 {
                chunk.set_block(ChunkBlockPos::new(x, 64, z), block!("stone"));
            }
        }
        chunk.set_block(
            ChunkBlockPos::new(8, 65, 8),
            fluid_block(FluidKind::Water, 0),
        );

        settle_chunk(
            &mut chunk,
            ChunkPos::new(0, 0),
            Dimension::Overworld,
            FluidAlgorithm::Vanilla,
            0,
        );

        // The source must have spread to its four horizontal neighbours on the floor.
        for (nx, nz) in [(7u8, 8u8), (9, 8), (8, 7), (8, 9)] {
            let b = chunk.get_block(ChunkBlockPos::new(nx, 65, nz));
            assert!(
                fluid_state(b).is_some_and(|s| s.kind == FluidKind::Water),
                "settle_chunk should have flowed water to ({nx},65,{nz}), got {b:?}"
            );
        }
    }

    /// `settle_chunk` is a cheap no-op on a chunk with no fluid.
    #[test]
    fn settle_chunk_noop_without_fluid() {
        let mut chunk = Chunk::new_empty();
        for x in 0..16u8 {
            for z in 0..16u8 {
                chunk.set_block(ChunkBlockPos::new(x, 64, z), block!("stone"));
            }
        }
        let before: Vec<_> = (0..16)
            .flat_map(|x| (0..16).map(move |z| (x as u8, z as u8)))
            .map(|(x, z)| chunk.get_block(ChunkBlockPos::new(x, 64, z)))
            .collect();
        settle_chunk(
            &mut chunk,
            ChunkPos::new(0, 0),
            Dimension::Overworld,
            FluidAlgorithm::Vanilla,
            0,
        );
        let after: Vec<_> = (0..16)
            .flat_map(|x| (0..16).map(move |z| (x as u8, z as u8)))
            .map(|(x, z)| chunk.get_block(ChunkBlockPos::new(x, 64, z)))
            .collect();
        assert_eq!(before, after, "a fluid-free chunk must be left untouched");
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
