use std::array::from_fn;

use crate::biome::Biome;
use crate::biome_chunk::{BiomeChunk, NoisePoint};
use crate::common::aquifer::FluidType;
use crate::common::carver::CarvingMask;
use crate::common::noise::generate_interpolation_data;
use crate::errors::WorldGenError;
use crate::overworld::aquifer::Aquifer;
use crate::overworld::carver::OverworldCarver;
use crate::overworld::noise_biome_parameters::overworld_biomes;
use crate::overworld::noise_depth::OverworldBiomeNoise;
use crate::overworld::spline::SplineCoord;
use crate::overworld::surface::OverworldSurface;
use crate::random::Xoroshiro128PlusPlus;
use bevy_math::DVec3;
use ferrumc_macros::block;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk_format::Chunk;
use ferrumc_world::pos::{
    BlockPos, ChunkBlockPos, ChunkColumnPos, ChunkHeight, ChunkPos, ColumnPos,
};
use itertools::Itertools;

pub(super) const CHUNK_HEIGHT: ChunkHeight = ChunkHeight::new(-64, 384);

#[derive(Clone, Copy)]
pub struct CachedNoise {
    pub jagged: f64,
    pub jaggedness: f64,
    pub factor: f64,
    pub offset: f64,
    pub temperature: f64,
    pub vegetation: f64,
    pub spline: SplineCoord,
}

impl CachedNoise {
    pub fn new(pos: ColumnPos, biome_noise: &OverworldBiomeNoise) -> Self {
        let pos = pos.block(0);
        let transformed = biome_noise.transform(pos.into());
        let spline_params = biome_noise.make_spline_params(transformed);
        let jaggedness = biome_noise.jaggedness(spline_params);
        let jagged = biome_noise
            .jagged
            .at(DVec3::from(pos) * DVec3::new(1500.0, 0.0, 1500.0));

        let factor = biome_noise.factor(spline_params);
        let offset = biome_noise.offset(spline_params);
        let temperature = biome_noise.temperature.at(transformed);
        let vegetation = biome_noise.vegetation.at(transformed);

        Self {
            spline: spline_params,
            temperature,
            vegetation,
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
    aquifer: Aquifer,
}

impl OverworldGenerator {
    pub fn new(_seed: u64) -> Self {
        let seed = 1;
        let random = Xoroshiro128PlusPlus::from_seed(seed).fork();
        let biome_noise = OverworldBiomeNoise::new(random);
        let aquifer = Aquifer::new(random);
        Self {
            seed,
            biome_seed: u64::from_be_bytes(
                cthash::sha2_256(&seed.to_be_bytes())[0..8]
                    .try_into()
                    .unwrap(),
            ),
            biome_noise,
            biomes: overworld_biomes(),
            aquifer,
            surface: OverworldSurface::new(random),
            carver: OverworldCarver::new(),
        }
    }

    pub fn generate_chunk(&self, chunk_pos: ChunkPos) -> Result<Chunk, WorldGenError> {
        let mut chunk = Chunk::new(CHUNK_HEIGHT);
        for i in -4..4 {
            chunk.set_section(
                i,
                if i < 0 {
                    block!("stone")
                } else {
                    block!("stone")
                },
            )?;
        }

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
                let cache_pos = (pos.pos - chunk_pos.origin().block(0).pos) / 4;
                self.biome_noise.pre_baked_final_density(
                    pos,
                    spline_coord_cache[cache_pos.x as usize][cache_pos.z as usize],
                )
            },
            chunk_pos,
            |pos, res| {
                let rel_pos = pos.chunk_block_pos();
                let res = res.min(self.biome_noise.noodle(pos.into()));
                if res > 0.0 {
                    if pos.y() >= 64 {
                        chunk.set_block(rel_pos, block!("stone")).unwrap();
                    }
                } else {
                    let fluid_type = if false {
                        self.aquifer.at(&self.biome_noise, pos, res).0
                    } else {
                        if res > 0. { None } else { Some(FluidType::Air) }
                    };
                    if pos.y() < 64 {
                        if let Some(block) = fluid_type {
                            chunk.set_block(rel_pos, block.into()).unwrap();
                        }
                    } else {
                        chunk
                            .set_block(
                                rel_pos,
                                fluid_type.map(|f| f.into()).unwrap_or(block!("stone")),
                            )
                            .unwrap();
                    }
                }
            },
        );
        if chunk_pos.pos.to_array() != [0, 0] {
            return Ok(chunk);
        }
        let biomes = BiomeChunk::generate(
            |pos| {
                let cache_pos = (pos.pos - chunk_pos.origin().block(0).pos) / 4;
                self.biome_noise.at_inner(
                    pos,
                    spline_coord_cache[cache_pos.x as usize][cache_pos.z as usize],
                )
            },
            self.seed,
            &self.biomes,
            chunk_pos,
            CHUNK_HEIGHT,
        );

        for (x, z) in (0..15).cartesian_product(0..15) {
            self.surface.build_surface(
                &self.biome_noise,
                &mut chunk,
                &biomes,
                chunk_pos.column_pos(ChunkColumnPos::new(x, z)),
            );
        }
        // self.carver.carve(
        //     &mut chunk,
        //     &biomes,
        //     self.seed,
        //     chunk_pos,
        //     &mut CarvingMask::new(CHUNK_HEIGHT),
        //     &self.surface,
        //     &self.biome_noise,
        // );
        Ok(chunk)
    }
}
