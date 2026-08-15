//! Cave carving (retained from FerrumC's previous generator; the upstream `nicer-terrain` branch
//! dropped caves, but we keep them so generated worlds still have underground space).
//!
//! Samples a coarse 3D noise grid and trilinearly interpolates it per block, carving air where the
//! noise exceeds a threshold — but never carving through fluid or directly under it (so caves do
//! not drain oceans/lakes).

use crate::WorldGenerator;
use crate::interp::{lerp, smoothstep};
use ferrumc_macros::{block, match_block};
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::{ChunkBlockPos, ChunkPos};

impl WorldGenerator {
    pub(crate) fn generate_caves(&self, chunk: &mut Chunk, chunk_pos: ChunkPos) {
        const STEP_XZ: i32 = 2;
        const STEP_Y: i32 = 8;
        const Y_MIN: i32 = -60;
        const Y_MAX: i32 = 100;
        let y_len = Y_MAX - Y_MIN;

        let gx = (16 / STEP_XZ + 1) as usize; // 9
        let gz = (16 / STEP_XZ + 1) as usize; // 9
        let gy = (y_len / STEP_Y + 1) as usize;

        // Sample coarse grid.
        let mut grid = vec![0.0f64; gx * gy * gz];
        let idx = |ix: usize, iy: usize, iz: usize| -> usize { (iy * gz + iz) * gx + ix };

        for ix in 0..gx {
            for iz in 0..gz {
                for iy in 0..gy {
                    let x = (ix as i32) * STEP_XZ;
                    let z = (iz as i32) * STEP_XZ;
                    let y = Y_MIN + (iy as i32) * STEP_Y;

                    let world_x = chunk_pos.x() * 16 + x;
                    let world_z = chunk_pos.z() * 16 + z;

                    let n = self.cave_noise(
                        f64::from(world_x) / 2.0,
                        f64::from(y) / 2.0,
                        f64::from(world_z) / 2.0,
                    );

                    grid[idx(ix, iy, iz)] = n;
                }
            }
        }

        // Fill blocks using interpolation within each cell.
        //
        // Trilinear interpolation is separable, so the X interpolation at the two bracketing Z
        // edges is computed once per column for every grid-Y level (`xz0`/`xz1`) instead of being
        // redone, together with eight grid fetches, for all 160 Y cells in the column. The inner Y
        // loop is then just two Y-lerps and one Z-lerp — the exact same operations, in the same
        // X→Y→Z order, that `trilerp` performed. The buffers are allocated once and reused.
        let mut xz0 = vec![0.0f64; gy];
        let mut xz1 = vec![0.0f64; gy];

        for x in 0..16i32 {
            for z in 0..16i32 {
                let base_ix = (x / STEP_XZ) as usize;
                let base_iz = (z / STEP_XZ) as usize;
                let ix0 = base_ix;
                let ix1 = (base_ix + 1).min(gx - 1);
                let iz0 = base_iz;
                let iz1 = (base_iz + 1).min(gz - 1);

                let tx = smoothstep(f64::from(x % STEP_XZ) / f64::from(STEP_XZ));
                let tz = smoothstep(f64::from(z % STEP_XZ) / f64::from(STEP_XZ));

                for iy in 0..gy {
                    xz0[iy] = lerp(grid[idx(ix0, iy, iz0)], grid[idx(ix1, iy, iz0)], tx);
                    xz1[iy] = lerp(grid[idx(ix0, iy, iz1)], grid[idx(ix1, iy, iz1)], tx);
                }

                for y in Y_MIN..Y_MAX {
                    let yy = y - Y_MIN;
                    let base_iy = (yy / STEP_Y) as usize;
                    let ty = smoothstep(f64::from(yy % STEP_Y) / f64::from(STEP_Y));

                    let iy0 = base_iy;
                    let iy1 = (base_iy + 1).min(gy - 1);

                    let y0 = lerp(xz0[iy0], xz0[iy1], ty);
                    let y1 = lerp(xz1[iy0], xz1[iy1], ty);
                    let cave_noise = lerp(y0, y1, tz);

                    if cave_noise > 0.6 {
                        let current_block =
                            chunk.get_block(ChunkBlockPos::new(x as u8, y as i16, z as u8));
                        // Don't carve through air/fluid, and don't carve a block that has fluid
                        // directly above it (avoids draining bodies of water from below).
                        if match_block!("air", current_block)
                            || match_block!("cave_air", current_block)
                            || match_block!("water", current_block)
                            || match_block!(
                                "water",
                                chunk.get_block(ChunkBlockPos::new(
                                    x as u8,
                                    (y + 1) as i16,
                                    z as u8
                                ))
                            )
                        {
                            continue;
                        }
                        chunk.set_block(
                            ChunkBlockPos::new(x as u8, y as i16, z as u8),
                            block!("air"),
                        );
                    }
                }
            }
        }
    }
}
