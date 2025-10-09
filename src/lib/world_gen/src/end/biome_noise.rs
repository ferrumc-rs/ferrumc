use bevy_math::{IVec2, Vec2};
use ferrumc_world::{chunk_format::Chunk, vanilla_chunk_format::BlockData};
use itertools::Itertools;
use std::{array::from_fn, f64};

use crate::{
    common::noise::{generate_interpolation_data, slide},
    perlin_noise::{BASE_3D_NOISE_END, BlendedNoise, ImprovedNoise},
    pos::{BlockPos, ChunkPos, ColumnPos},
    random::LegacyRandom,
};
use std::f32;

pub struct EndNoise {
    island_simplex: ImprovedNoise,
    base_noise: BlendedNoise,
}
impl EndNoise {
    pub fn new(seed: u64) -> Self {
        let mut random = LegacyRandom::new(seed);
        let mut noise_random = LegacyRandom::new(0);
        noise_random.advance(17292);

        Self {
            base_noise: BASE_3D_NOISE_END.init(&mut random),
            island_simplex: ImprovedNoise::new(&mut noise_random),
        }
    }
    pub fn generate_chunk(&self, pos: ChunkPos, chunk: &mut Chunk) {
        let islands_cache: [[f64; 3]; 3] =
            from_fn(|x| from_fn(|z| self.islands(pos.column_pos(x as u32 * 8, z as u32 * 8))));
        generate_interpolation_data(
            |pos| self.pre_backed_final_density(islands_cache, pos),
            chunk,
            pos,
            BlockData {
                name: "minecraft:end_stone".to_string(),
                properties: None,
            }
            .to_block_id(),
        );
    }

    fn pre_backed_final_density(&self, islands_cache: [[f64; 3]; 3], pos: BlockPos) -> f64 {
        let sloped_cheese = islands_cache[(pos.x as usize & 15) / 8][(pos.z as usize & 15) / 8]
            + self.base_noise.at(pos.as_dvec3() * 0.25 * 684.412);
        slide(
            pos.y.into(),
            sloped_cheese,
            128. - 72.,
            128. + 184.,
            -23.4375,
            4.,
            32.,
            -0.234375,
        )
    }

    fn get_height_value(&self, pos: IVec2) -> f32 {
        let pos_div_2 = pos / 2;
        let pos_parity = pos % 2;

        let mut res = pos.as_vec2().length() * 8.0;

        // if (pos_div_2.abs() + IVec2::splat(12)).length_squared() > 4096 {
        for dpos in (-12..=12).cartesian_product(-12..=12).map(IVec2::from) {
            let currpos = pos_div_2 + dpos;
            if currpos.length_squared() > 4096
                && self.island_simplex.legacy_simplex_at(currpos.as_dvec2()) < -0.9
            {
                // has to be cast because of float inaccuracies.
                let tmp = currpos.abs().as_vec2() * Vec2::new(3439., 147.);
                let f1 = (tmp.element_sum()) % 13. + 9.;

                let f4 = (dpos * 2 - pos_parity).as_vec2().length() * f1;
                res = res.min(f4);
            }
        }
        // }
        (100. - res).clamp(-100., 80.)
    }

    fn islands(&self, pos: ColumnPos) -> f64 {
        (f64::from(self.get_height_value(pos.pos / 8)) - 8.) / 128.
    }
}

#[test]
fn test_islands() {
    let noise = EndNoise::new(0);
    assert_eq!(noise.get_height_value(IVec2::new(0, 0)), 80.);
    assert_eq!(noise.get_height_value(IVec2::new(10000, -20031)), 57.51471);
}
