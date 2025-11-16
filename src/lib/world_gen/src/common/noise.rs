use std::mem::swap;

use bevy_math::FloatExt;
use ferrumc_world::{block_id::BlockId, chunk_format::Chunk};

use crate::{
    common::math::clamped_map,
    pos::{BlockPos, ChunkPos},
};

pub fn post_process(interpolated: f64) -> f64 {
    let d = (interpolated * 0.64).clamp(-1.0, 1.0);
    d / 2.0 - d * d * d / 24.0
}

#[expect(clippy::too_many_arguments)]
pub fn slide(
    y: f64,
    density: f64,
    top_start: f64,
    top_end: f64,
    top_delta: f64,
    bottom_start: f64,
    bottom_end: f64,
    bottom_delta: f64,
) -> f64 {
    let s = clamped_map(y, top_start, top_end, 1.0, 0.0);
    let t = clamped_map(y, bottom_start, bottom_end, 0.0, 1.0);
    bottom_delta.lerp(top_delta.lerp(density, s), t)
}
pub fn generate_interpolation_data(
    get: impl Fn(BlockPos) -> f64,
    chunk: &mut Chunk,
    pos: ChunkPos,
    filler: BlockId,
) {
    const WIDTH: usize = 8;
    const HEIGHT: i32 = 4;
    const MIN_Y: i32 = 0;
    const CHUNK_HEIGHT: i32 = 256;
    const CHUNK_WIDTH: usize = 16;
    const SECTIONS_HORIZONTAL: usize = CHUNK_WIDTH / WIDTH;
    const SAMPLES_HORIZONTAL: usize = SECTIONS_HORIZONTAL + 1;
    const SECTIONS_VERTICAL: i32 = CHUNK_HEIGHT / HEIGHT;
    const SAMPLES_VERTICAL: i32 = SECTIONS_VERTICAL + 1;

    let mut slice0 = [[0.0; SAMPLES_HORIZONTAL]; SAMPLES_HORIZONTAL];
    let mut slice1 = [[0.0; SAMPLES_HORIZONTAL]; SAMPLES_HORIZONTAL];
    // y = 0
    for (x, slice1x) in slice1.iter_mut().enumerate() {
        for (z, slice1xz) in slice1x.iter_mut().enumerate() {
            *slice1xz = get(pos.block((x * WIDTH) as u32, MIN_Y, (z * WIDTH) as u32));
        }
    }

    for y in 1..SAMPLES_VERTICAL {
        swap(&mut slice0, &mut slice1);

        // x = 0
        for z in 0..SAMPLES_HORIZONTAL {
            slice1[0][z] = get(pos.block(0, y * HEIGHT + MIN_Y, (z * WIDTH) as u32));
        }

        for x in 1..SAMPLES_HORIZONTAL {
            // z = 0;
            slice1[x][0] = get(pos.block((x * WIDTH) as u32, y * HEIGHT + MIN_Y, 0));
            for z in 1..SAMPLES_HORIZONTAL {
                slice1[x][z] =
                    get(pos.block((x * WIDTH) as u32, y * HEIGHT + MIN_Y, (z * WIDTH) as u32));
                // if x != 1 || z != 1 || y != 64 / HEIGHT || pos.pos.x != 0 || pos.pos.y != 0 {
                //     continue;
                // }

                // let p000 = 10.;
                // let p001 = -1.;
                // let p100 = -1.;
                // let p101 = -1.;
                // let p010 = -1.;
                // let p011 = -1.;
                // let p110 = -1.;
                // let p111 = 10.;
                let p000 = slice0[x - 1][z - 1];
                let p001 = slice0[x - 1][z];
                let p100 = slice0[x][z - 1];
                let p101 = slice0[x][z];
                let p010 = slice1[x - 1][z - 1];
                let p011 = slice1[x - 1][z];
                let p110 = slice1[x][z - 1];
                let p111 = slice1[x][z];

                for cy in 0..HEIGHT {
                    let fy = f64::from(cy) / f64::from(HEIGHT);
                    let value_xz00 = p000.lerp(p010, fy);
                    let value_xz10 = p100.lerp(p110, fy);
                    let value_xz01 = p001.lerp(p011, fy);
                    let value_xz11 = p101.lerp(p111, fy);
                    for cx in 0..WIDTH as u32 {
                        let fx = f64::from(cx) / WIDTH as f64;
                        let value_z0 = value_xz00.lerp(value_xz10, fx);
                        let value_z1 = value_xz01.lerp(value_xz11, fx);
                        for cz in 0..WIDTH as u32 {
                            let fz = f64::from(cz) / WIDTH as f64;
                            let value = value_z0.lerp(value_z1, fz);

                            let res = post_process(value);

                            let pos = pos.block(
                                cx + (x as u32 - 1) * WIDTH as u32,
                                cy + (y - 1) * HEIGHT + MIN_Y,
                                cz + (z as u32 - 1) * WIDTH as u32,
                            );

                            if res > 0.0 {
                                chunk.set_block(pos, filler).unwrap();
                            }
                        }
                    }
                }
            }
        }
    }
}
