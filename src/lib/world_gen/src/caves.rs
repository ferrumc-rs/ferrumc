use crate::interp::{smoothstep, trilerp};
use ferrumc_macros::{block, match_block};
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::{ChunkBlockPos, ChunkPos};

pub(crate) fn generate_caves(
    chunk: &mut Chunk,
    chunk_pos: ChunkPos,
    generator: &super::NoiseGenerator,
) {
    const STEP_XZ: i32 = 2;
    const STEP_Y: i32 = 6;

    // Your y loop is -60..100 (160 tall)
    const Y_MIN: i32 = -60;
    const Y_MAX: i32 = 100;
    let y_len = Y_MAX - Y_MIN;

    let gx = (16 / STEP_XZ + 1) as usize; // 5
    let gz = (16 / STEP_XZ + 1) as usize; // 5
    let gy = (y_len / STEP_Y + 1) as usize; // 41

    // Sample coarse grid
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

                let n = generator.get_cave_noise(
                    f64::from(world_x) / 2.0,
                    f64::from(y) / 2.0,
                    f64::from(world_z) / 2.0,
                );

                grid[idx(ix, iy, iz)] = n;
            }
        }
    }

    // Now fill blocks using interpolation within each cell
    for x in 0..16i32 {
        for z in 0..16i32 {
            let base_ix = (x / STEP_XZ) as usize;
            let base_iz = (z / STEP_XZ) as usize;

            let tx = smoothstep(f64::from(x % STEP_XZ) / f64::from(STEP_XZ));
            let tz = smoothstep(f64::from(z % STEP_XZ) / f64::from(STEP_XZ));

            for y in Y_MIN..Y_MAX {
                
                let yy = y - Y_MIN;
                let base_iy = (yy / STEP_Y) as usize;
                let ty = smoothstep(f64::from(yy % STEP_Y) / f64::from(STEP_Y));

                // Read 8 corners
                let ix0 = base_ix;
                let ix1 = (base_ix + 1).min(gx - 1);
                let iz0 = base_iz;
                let iz1 = (base_iz + 1).min(gz - 1);
                let iy0 = base_iy;
                let iy1 = (base_iy + 1).min(gy - 1);

                let c000 = grid[idx(ix0, iy0, iz0)];
                let c100 = grid[idx(ix1, iy0, iz0)];
                let c010 = grid[idx(ix0, iy1, iz0)];
                let c110 = grid[idx(ix1, iy1, iz0)];
                let c001 = grid[idx(ix0, iy0, iz1)];
                let c101 = grid[idx(ix1, iy0, iz1)];
                let c011 = grid[idx(ix0, iy1, iz1)];
                let c111 = grid[idx(ix1, iy1, iz1)];

                let cave_noise =
                    trilerp(c000, c100, c010, c110, c001, c101, c011, c111, tx, ty, tz);

                // Carving logic
                if cave_noise > 0.6 {
                    let current_block = chunk.get_block(ChunkBlockPos::new(x as u8, y as i16, z as u8));
                    if match_block!("air", current_block)
                        || match_block!("cave_air", current_block)
                        || match_block!("water", current_block)
                        || match_block!(
                        "water",
                        chunk.get_block(ChunkBlockPos::new(x as u8, (y + 1) as i16, z as u8))
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
