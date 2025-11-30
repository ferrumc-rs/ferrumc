use std::mem::swap;

use bevy_math::{FloatExt, U8Vec3};

use crate::{
    common::math::clamped_map,
    pos::{BlockPos, ChunkBlockPos, ChunkPos},
};

pub fn post_process(interpolated: f64) -> f64 {
    let d = (interpolated * 0.64).clamp(-1.0, 1.0);
    d / 2.0 - d * d * d / 24.0
}

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
    noise: impl Fn(BlockPos) -> f64,
    pos: ChunkPos,
    mut action: impl FnMut(BlockPos, f64),
) {
    const WIDTH: usize = 4;
    const HEIGHT: i32 = 8;
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
            *slice1xz = noise(pos.block_offset((x * WIDTH) as i32, MIN_Y, (z * WIDTH) as i32));
        }
    }

    for y in 1..SAMPLES_VERTICAL {
        swap(&mut slice0, &mut slice1);

        // x = 0
        for z in 0..SAMPLES_HORIZONTAL {
            slice1[0][z] = noise(pos.block_offset(0, y * HEIGHT + MIN_Y, (z * WIDTH) as i32));
        }

        for x in 1..SAMPLES_HORIZONTAL {
            // z = 0;
            slice1[x][0] = noise(pos.block_offset((x * WIDTH) as i32, y * HEIGHT + MIN_Y, 0));
            for z in 1..SAMPLES_HORIZONTAL {
                slice1[x][z] = noise(pos.block_offset(
                    (x * WIDTH) as i32,
                    y * HEIGHT + MIN_Y,
                    (z * WIDTH) as i32,
                ));
                let p000 = slice0[x - 1][z - 1];
                let p001 = slice0[x - 1][z];
                let p100 = slice0[x][z - 1];
                let p101 = slice0[x][z];
                let p010 = slice1[x - 1][z - 1];
                let p011 = slice1[x - 1][z];
                let p110 = slice1[x][z - 1];
                let p111 = slice1[x][z];

                let block_x = (x as u8 - 1) * WIDTH as u8;
                let block_y = (y - 1) * HEIGHT + MIN_Y;
                let block_z = (z as u8 - 1) * WIDTH as u8;
                let base_pos = pos.chunk_block((block_x, block_y as i16, block_z).into());
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

                            let curr_pos = ChunkBlockPos::from(
                                base_pos + BlockPos::new(cx as i32, cy, cz as i32),
                            );

                            action(pos.chunk_block(curr_pos), res);
                        }
                    }
                }
            }
        }
    }
}
