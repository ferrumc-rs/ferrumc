use bevy_math::IVec2;
use ferrumc_world::chunk_format::Chunk;

use crate::{
    errors::WorldGenError, nether::noise::NetherNoise, pos::{ChunkHeight, ChunkPos}
};

pub struct NetherGenerator {
    seed: u64,
    biome_seed: u64,
    chunk_height: ChunkHeight,
    noise: NetherNoise,
}

impl NetherGenerator {
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
            noise: NetherNoise::new(seed),
        }
    }

    pub fn generate_chunk(&self, x: i32, z: i32) -> Result<Chunk, WorldGenError> {
        let mut chunk = Chunk::new(x, z, "overworld".to_string());
        self.noise
            .generate_chunk(ChunkPos::from(IVec2::new(x * 16, z * 16)), &mut chunk);

        Ok(chunk)
    }
}
