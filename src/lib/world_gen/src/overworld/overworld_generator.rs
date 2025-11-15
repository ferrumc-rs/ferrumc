use std::array::from_fn;

use crate::biome::Biome;
use crate::biome_chunk::{BiomeChunk, NoisePoint};
use crate::common::noise::generate_interpolation_data;
use crate::errors::WorldGenError;
use crate::overworld::carver::OverworldCarver;
use crate::overworld::noise_biome_parameters::overworld_biomes;
use crate::overworld::noise_depth::OverworldBiomeNoise;
use crate::overworld::surface::OverworldSurface;
use crate::pos::{ChunkBlockPos, ChunkHeight, ChunkPos, ColumnPos};
use crate::random::Xoroshiro128PlusPlus;
use bevy_math::DVec3;
use ferrumc_macros::block;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk_format::Chunk;

pub(super) const CHUNK_HEIGHT: ChunkHeight = ChunkHeight::new(-64, 380);

#[derive(Clone, Copy)]
pub struct CachedNoise {
    pub jagged: f64,
    pub jaggedness: f64,
    pub factor: f64,
    pub offset: f64,
}

impl CachedNoise {
    pub fn new(pos: ColumnPos, biome_noise: &OverworldBiomeNoise) -> Self {
        let pos = pos.block(0);
        let spline_params = biome_noise.make_spline_params(biome_noise.transform(pos.as_dvec3()));
        let jaggedness = biome_noise.jaggedness(spline_params);
        let jagged = biome_noise
            .jagged
            .at(pos.as_dvec3() * DVec3::new(1500.0, 0.0, 1500.0));

        let factor = biome_noise.factor(spline_params);
        let offset = biome_noise.offset(spline_params);
        Self {
            jagged,
            jaggedness,
            factor,
            offset,
        }
    }
}

pub struct OverworldGenerator {
    seed: u64,
    biome_seed: u64,
    biome_noise: OverworldBiomeNoise,
    biomes: Vec<(NoisePoint, Biome)>,
    surface: OverworldSurface,
    carver: OverworldCarver,
}

impl OverworldGenerator {
    pub fn new(_seed: u64) -> Self {
        let seed = 1;
        let random = Xoroshiro128PlusPlus::from_seed(seed).fork();
        let biome_noise = OverworldBiomeNoise::new(random);
        Self {
            seed,
            biome_seed: u64::from_be_bytes(
                cthash::sha2_256(&seed.to_be_bytes())[0..8]
                    .try_into()
                    .unwrap(),
            ),
            biome_noise,
            biomes: overworld_biomes(),
            surface: OverworldSurface::new(random),
            carver: OverworldCarver::new(),
        }
    }

    fn generate_biomes(&self, pos: ChunkPos) -> BiomeChunk {
        BiomeChunk::generate(
            &self.biome_noise,
            self.seed,
            &self.biomes,
            pos,
            CHUNK_HEIGHT,
        )
    }

    pub fn generate_chunk(&self, x: i32, z: i32) -> Result<Chunk, WorldGenError> {
        let mut chunk = Chunk::new(x, z, "overworld".to_string());
        for i in -4..4 {
            chunk.set_section(
                i,
                if i < 0 {
                    block!("deepslate", {axis: "x"})
                } else {
                    block!("stone")
                },
            )?;
        }

        let chunk_pos = ChunkPos::new(x * 16, z * 16);
        let spline_coord_cache: [[CachedNoise; 5]; 5] = from_fn(|x| {
            from_fn(|z| {
                CachedNoise::new(
                    chunk_pos.column_offset(x as i32 * 4, z as i32 * 4),
                    &self.biome_noise,
                )
            })
        });
        generate_interpolation_data(
            |pos| {
                let cache_pos = (pos - chunk_pos.origin().block(0)) / 4;
                self.biome_noise.pre_baked_final_density(
                    pos,
                    spline_coord_cache[cache_pos.x as usize][cache_pos.z as usize],
                )
            },
            chunk_pos,
            |pos, res| {
                let rel_pos = ChunkBlockPos::from(pos);

                if res.min(self.biome_noise.noodle(pos.as_dvec3())) > 0.0 {
                    if pos.y >= 64 {
                        chunk
                            .set_block(
                                i32::from(rel_pos.pos.x),
                                i32::from(rel_pos.pos.y),
                                i32::from(rel_pos.pos.z),
                                block!("stone"),
                            )
                            .unwrap();
                    }
                } else if pos.y < 64 {
                    chunk
                        .set_block(
                            i32::from(rel_pos.pos.x),
                            i32::from(rel_pos.pos.y),
                            i32::from(rel_pos.pos.z),
                            block!("air"),
                        )
                        .unwrap();
                }
            },
        );
        Ok(chunk)
    }
}
