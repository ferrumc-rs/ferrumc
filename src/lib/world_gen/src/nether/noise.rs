use bevy_math::DVec3;
use ferrumc_world::{chunk_format::Chunk, vanilla_chunk_format::BlockData};
use std::f64;

use crate::{
    common::noise::{generate_interpolation_data, slide},
    perlin_noise::{BASE_3D_NOISE_NETHER, BlendedNoise},
    pos::{BlockPos, ChunkPos},
    random::LegacyRandom,
};

pub struct NetherNoise {
    base_noise: BlendedNoise,
}
impl NetherNoise {
    pub fn new(seed: u64) -> Self {
        let mut random = LegacyRandom::new(seed);

        Self {
            base_noise: BASE_3D_NOISE_NETHER.init(&mut random),
        }
    }
    pub fn generate_chunk(&self, pos: ChunkPos, chunk: &mut Chunk) {
        generate_interpolation_data(
            |pos| self.pre_backed_final_density(pos),
            chunk,
            pos,
            BlockData {
                name: "minecraft:end_stone".to_string(),
                properties: None,
            }
            .to_block_id(),
        );
    }

    fn pre_backed_final_density(&self, pos: BlockPos) -> f64 {
        let sloped_cheese = self
            .base_noise
            .at(pos.as_dvec3() * DVec3::new(0.25, 0.375, 0.25) * 684.412);
        slide(
            pos.y.into(),
            sloped_cheese,
            128. - 24.,
            128.,
            0.9375,
            -8.,
            24.,
            2.5,
        )
    }
}
