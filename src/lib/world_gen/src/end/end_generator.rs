use bevy_math::IVec2;
use ferrumc_world::chunk_format::Chunk;

use crate::{
    end::biome_noise::EndNoise,
    errors::WorldGenError,
    pos::{ChunkHeight, ChunkPos},
};

pub struct EndGenerator {
    seed: u64,
    biome_seed: u64,
    chunk_height: ChunkHeight,
    biome_noise: EndNoise,
}

impl EndGenerator {
    pub fn new(_seed: u64) -> Self {
        let seed = 1;
        // let random = Xoroshiro128PlusPlus::from_seed(seed).fork();
        let chunk_height = ChunkHeight {
            min_y: 0,
            height: 256,
        };
        Self {
            seed,
            biome_seed: u64::from_be_bytes(
                cthash::sha2_256(&seed.to_be_bytes())[0..8]
                    .try_into()
                    .unwrap(),
            ),
            chunk_height,
            biome_noise: EndNoise::new(seed),
        }
    }

    pub fn generate_chunk(&self, x: i32, z: i32) -> Result<Chunk, WorldGenError> {
        let mut chunk = Chunk::new(x, z, "overworld".to_string());
        self.biome_noise
            .generate_chunk(ChunkPos::from(IVec2::new(x * 16, z * 16)), &mut chunk);
        // .iter_columns()
        // .cartesian_product(self.chunk_height.iter())
        // .map(|(c, y)| c.block(y))
        // .map(|pos| {
        //     let final_density = self.biome_noise.final_density(pos);
        //     chunk.set_block(
        //         pos.x,
        //         pos.y,
        //         pos.z,
        //         if final_density > 0.0 { stone } else { air },
        //     )
        // })
        // .find(Result::is_err)
        // .unwrap_or(Ok(()))?;

        Ok(chunk)
    }
}
