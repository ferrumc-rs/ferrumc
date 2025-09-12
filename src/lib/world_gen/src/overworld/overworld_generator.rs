use crate::biome::Biome;
use crate::biome_chunk::{BiomeChunk, BiomeNoise, NoisePoint};
use crate::overworld::aquifer::clamped_map;
use crate::overworld::carver::OverworldCarver;
use crate::overworld::noise_biome_parameters::overworld_biomes;
use crate::overworld::noise_depth::{get_depth_spline, overworld_factor, overworld_jaggedness};
use crate::overworld::spline::CubicSpline;
use crate::overworld::surface::OverworldSurface;
use crate::perlin_noise::{
    CAVE_CHEESE, CAVE_ENTRANCE, CAVE_LAYER, CONTINENTALNESS, EROSION, JAGGED, NOODLE,
    NOODLE_RIDGE_A, NOODLE_RIDGE_B, NOODLE_THICKNESS, NormalNoise, PILLAR, PILLAR_RARENESS, RIDGE,
    SHIFT, SPAGHETTI_2D, SPAGHETTI_2D_ELEVATION, SPAGHETTI_2D_MODULATOR, SPAGHETTI_2D_THICKNESS,
    SPAGHETTI_3D_1, SPAGHETTI_3D_2, SPAGHETTI_3D_RARITY, SPAGHETTI_3D_THICKNESS,
    SPAGHETTI_ROUGHNESS, SPAGHETTI_ROUGHNESS_MODULATOR, TEMPERATURE, VEGETATION,
};
use crate::pos::{BlockPos, ChunkHeight, ChunkPos};
use crate::random::{RandomFactory, Rng, Xoroshiro128PlusPlus};
use bevy_math::{DVec3, FloatExt, Vec3Swizzles};
use ferrumc_world::chunk_format::Chunk;

pub struct OverworldGenerator {
    seed: u64,
    chunk_height: ChunkHeight,
    biome_noise: OverworldBiomeNoise,
    biomes: Vec<(NoisePoint, Biome)>,
    surface: OverworldSurface,
    carver: OverworldCarver,
}

const BLEND_ALPHA: f64 = 1.0;
const BLEND_OFFSET: f64 = 0.0;
pub(super) struct OverworldBiomeNoise {
    depth: CubicSpline,
    shift: NormalNoise<4>,
    temperature: NormalNoise<6>,
    vegetation: NormalNoise<6>,
    continents: NormalNoise<9>,
    erosion: NormalNoise<5>,
    ridges: NormalNoise<6>,
    jagged: NormalNoise<16>,
}
impl OverworldBiomeNoise {
    fn transform(&self, pos: BlockPos) -> DVec3 {
        let shift_x = self.shift.get_value(pos.with_y(0).into());
        let shift_z = self.shift.get_value(pos.with_y(0).zxy().into());
        pos.as_dvec3() * DVec3::new(0.25, 1.0, 0.25) + DVec3::new(shift_x, 0.0, shift_z)
    }
    //TODO: move
    fn initial_density_without_jaggedness(&self, pos: BlockPos) -> f64 {
        let factor = self.factor(pos);
        let mut factor_depth = factor * self.depth(pos);
        factor_depth *= if factor_depth > 0.0 { 4.0 } else { 1.0 };
        let density = (factor_depth - 0.703125).clamp(-64.0, 64.0);
        slide(
            pos.y, density, -64, 384, 80, 64, -0.078125, 0, 24, 0.1171875,
        )
    }

    fn factor(&self, pos: BlockPos) -> f64 {
        let factor = overworld_factor();
        let ridges = self.ridges(pos);
        let ridges_folded = ((ridges.abs() - 0.6666666666666666).abs() - 0.3333333333333333) * -3.0;
        let erosion = self.erosion(pos);
        let continents = self.continents(pos);
        let factor = 10.0.lerp(
            factor.sample(
                continents as f32,
                erosion as f32,
                ridges_folded as f32,
                ridges as f32,
            ) as f64,
            BLEND_ALPHA,
        );
        factor
    }

    fn final_density(&self, pos: BlockPos) -> f64 {
        let factor = overworld_factor();
        let ridges = self.ridges(pos);
        let ridges_folded = ((ridges.abs() - 0.6666666666666666).abs() - 0.3333333333333333) * -3.0;
        let erosion = self.erosion(pos);
        let continents = self.continents(pos);
        let jaggedness = 0.0.lerp(
            overworld_jaggedness().sample(
                continents as f32,
                erosion as f32,
                ridges_folded as f32,
                ridges as f32,
            ) as f64,
            BLEND_ALPHA,
        );
        let random = todo!();
        let jagged = JAGGED
            .init(random)
            .get_value(pos.as_dvec3() * DVec3::new(1500.0, 0.0, 1500.0));
        let jagged_tmp = jagged * if jagged > 0.0 { 1.0 } else { 0.5 } * jaggedness;
        let b = self.offset(pos) + (pos.y as f64).remap(-64.0, 320.0, 1.5, -1.5);
        // underground, ENTANCES
        let c = self.factor(pos) * (self.depth(pos) + jagged_tmp);
        let BASE_3D_NOISE_OVERWORLD = todo!();
        let sloped_cheese = c * if c > 0.0 { 4.0 } else { 1.0 } + BASE_3D_NOISE_OVERWORLD;
        fn entrances(pos: DVec3) -> f64 {
            let rarity = SPAGHETTI_3D_RARITY
                .init(random)
                .get_value(pos * DVec3::new(2.0, 1.0, 2.0));
            let rarity = if rarity < -0.5 {
                0.75
            } else if rarity < 0.0 {
                1.0
            } else if rarity < 0.5 {
                1.5
            } else {
                2.0
            };
            let spaghetti_3d_thickness = SPAGHETTI_3D_THICKNESS
                .init(random)
                .get_value(pos)
                .remap(-1.0, 1.0, -0.065, -0.088);
            let spaghetti_3d_1 = SPAGHETTI_3D_1.init(random).get_value(pos / rarity).abs() * rarity;
            let spaghetti_3d_2 = SPAGHETTI_3D_2.init(random).get_value(pos / rarity).abs() * rarity;
            let spaghetti_3d =
                (spaghetti_3d_1.max(spaghetti_3d_2) + spaghetti_3d_thickness).clamp(-1.0, 1.0);
            let initial_spaghetti_roughness = SPAGHETTI_ROUGHNESS.init(random).get_value(pos);
            let spaghetti_roughness_modulator = SPAGHETTI_ROUGHNESS_MODULATOR
                .init(random)
                .get_value(pos)
                .remap(-1.0, 1.0, 0.0, -0.1);
            let spaghetti_roughness =
                (initial_spaghetti_roughness.abs() - 0.4) * spaghetti_roughness_modulator;
            let cave_entrance = CAVE_ENTRANCE
                .init(random)
                .get_value(pos * DVec3::new(0.75, 0.5, 0.75));
            let tmp = cave_entrance + 0.37 + clamped_map(pos.y, -10.0, 30.0, 0.3, 0.0);
            tmp.min(spaghetti_roughness + spaghetti_3d)
        }
        fn spaghetti_2d(pos: DVec3) -> f64 {
            let spaghetti_roughness_modulator = SPAGHETTI_2D_MODULATOR
                .init(random)
                .get_value(pos * DVec3::new(2.0, 1.0, 2.0));
            let rarity = if spaghetti_roughness_modulator < -0.75 {
                0.5
            } else if spaghetti_roughness_modulator < -0.5 {
                0.75
            } else if spaghetti_roughness_modulator < 0.5 {
                1.0
            } else if spaghetti_roughness_modulator < 0.75 {
                2.0
            } else {
                3.0
            };
            let spaghetti_2d = SPAGHETTI_2D.init(random).get_value(pos / rarity).abs() * rarity;
            let spaghetti_2d_elevation = SPAGHETTI_2D_ELEVATION.init(random).get_value(pos).remap(
                -1.0,
                1.0,
                -64i32.div_euclid(8) as f64,
                8.0,
            );
            let tmp = (spaghetti_2d_elevation + clamped_map(pos.y, -64.0, 320.0, 8.0, -40.0)).abs();
            let spaghetti_2d_thickness_modulator = SPAGHETTI_2D_THICKNESS
                .init(random)
                .get_value(pos * DVec3::new(2.0, 1.0, 2.0))
                .remap(-1.0, 1.0, -0.6, -1.3);
            let thickness = (tmp + spaghetti_2d_thickness_modulator).powi(3);
            let tmp2 = spaghetti_2d + 0.083 * spaghetti_2d_thickness_modulator;
            thickness.max(tmp2).clamp(-1.0, 1.0)
        }
        fn pillars(pos: DVec3) -> f64 {
            let pillar = PILLAR
                .init(random)
                .get_value(pos * DVec3::new(25.0, 0.3, 25.0));
            let pillar_rareness = PILLAR_RARENESS
                .init(random)
                .get_value(pos)
                .remap(-1.0, 1.0, 0.0, -2.0);
            let pillar_thickness = PILLAR_RARENESS
                .init(random)
                .get_value(pos)
                .remap(-1.0, 1.0, 0.0, 1.1);
            pillar_thickness.powi(3) * (pillar * 2.0 + pillar_rareness)
        }
        fn underground(sloped_cheese: f64, pos: DVec3) -> f64 {
            let spaghetti_2d = spaghetti_2d(pos);
            let initial_spaghetti_roughness = SPAGHETTI_ROUGHNESS.init(random).get_value(pos);
            let spaghetti_roughness_modulator = SPAGHETTI_ROUGHNESS_MODULATOR
                .init(random)
                .get_value(pos)
                .remap(-1.0, 1.0, 0.0, -0.1);
            let spaghetti_roughness =
                (initial_spaghetti_roughness.abs() - 0.4) * spaghetti_roughness_modulator;
            let cave_layer = CAVE_LAYER
                .init(random)
                .get_value(pos * DVec3::new(1.0, 8.0, 1.0));
            let tmp = cave_layer.powi(2) * 4.0;
            let cave_cheese = CAVE_CHEESE
                .init(random)
                .get_value(pos * DVec3::new(1.0, 0.6666666666666666, 1.0));
            let tmp2 = (cave_cheese + 0.27).clamp(-1.0, 1.0)
                + (1.5 + sloped_cheese * -0.64).clamp(0.0, 0.5);
            let f4 = tmp2 + tmp;
            let f5 = f4
                .min(entrances(pos))
                .min(spaghetti_roughness + spaghetti_2d);
            let pillars = pillars(pos);
            if pillars <= 0.03 { f5 } else { f5.max(pillars) }
        }

        let f7 = sloped_cheese.min(5.0 * entrances(pos.into()));
        let f8 = if sloped_cheese < 1.5625 {
            f7
        } else {
            underground(sloped_cheese, pos.into())
        };

        let tmp = slide(pos.y, f8, -64, 384, 80, 64, -0.078125, 0, 24, 0.1171875);
        let blended = blender.blend_density(pos, tmp); //interpolated

        let tmp2 = (blended * 0.64).clamp(-1.0, 1.0);
        fn noodle(pos: DVec3) -> f64 {
            if pos.y < -60 {
                return 64.0;
            }
            let noodle = NOODLE.init(random).get_value(pos);
            let noodle_thickness = NOODLE_THICKNESS
                .init(random)
                .get_value(pos)
                .remap(-1.0, 1.0, -0.05, -0.1);
            let noodle_ridge_a = NOODLE_RIDGE_A
                .init(random)
                .get_value(pos * 2.6666666666666665);
            let noodle_ridge_b = NOODLE_RIDGE_B
                .init(random)
                .get_value(pos * 2.6666666666666665);
            let noodle_ridge = noodle_ridge_a.abs().max(noodle_ridge_b.abs()) * 1.5;
            if noodle <= 0 {
                64.0
            } else {
                noodle_thickness + noodle_ridge
            }
        }
        (d / 2.0 - d * d * d / 24.0).min(noodle(pos))
    }

    fn offset(&self, pos: BlockPos) -> f64 {
        let ridges = self.ridges(pos);
        let ridges_folded = ((ridges.abs() - 0.6666666666666666).abs() - 0.3333333333333333) * -3.0;
        let erosion = self.erosion(pos);
        let continents = self.continents(pos);
        let offset = BLEND_OFFSET.lerp(
            -0.50375
                + self.depth.sample(
                    continents as f32,
                    erosion as f32,
                    ridges_folded as f32,
                    ridges as f32,
                ) as f64,
            BLEND_ALPHA,
        );
        offset
    }
}

fn slide(
    y: i32,
    density: f64,
    min_y: i32,
    height: i32,
    top_start_offset: i32,
    top_end_offset: i32,
    top_delta: f64,
    bottom_start_offset: i32,
    bottom_end_offset: i32,
    bottom_delta: f64,
) -> f64 {
    let a = clamped_map(
        y as f64,
        (min_y + height - top_start_offset) as f64,
        (min_y + height - top_end_offset) as f64,
        1.0,
        0.0,
    );
    let l1 = top_delta.lerp(density, a);
    let b = clamped_map(
        y as f64,
        (min_y + bottom_start_offset) as f64,
        (min_y + bottom_end_offset) as f64,
        0.0,
        1.0,
    );
    let l2 = bottom_delta.lerp(l1, b);
    l2
}
impl BiomeNoise for OverworldBiomeNoise {
    fn temperature(&self, pos: BlockPos) -> f64 {
        self.temperature.get_value(self.transform(pos))
    }

    fn vegetation(&self, pos: BlockPos) -> f64 {
        self.vegetation.get_value(self.transform(pos))
    }

    fn continents(&self, pos: BlockPos) -> f64 {
        self.continents.get_value(self.transform(pos))
    }

    fn erosion(&self, pos: BlockPos) -> f64 {
        self.erosion.get_value(self.transform(pos))
    }

    fn depth(&self, pos: BlockPos) -> f64 {
        let offset = self.offset(pos);
        (pos.y as f64).remap(-64.0, 320.0, 1.5, -1.5) + offset
    }

    fn ridges(&self, pos: BlockPos) -> f64 {
        self.ridges.get_value(self.transform(pos))
    }
}

impl OverworldGenerator {
    pub fn new(seed: u64) -> Self {
        let rng_unwrapped = Xoroshiro128PlusPlus::from_seed(seed).fork_positional();
        let random = RandomFactory::Xoroshiro128PlusPlus(rng_unwrapped);
        let biome_noise = OverworldBiomeNoise {
            depth: get_depth_spline(),
            shift: SHIFT.init(random),
            temperature: TEMPERATURE.init(random),
            vegetation: VEGETATION.init(random),
            continents: CONTINENTALNESS.init(random),
            erosion: EROSION.init(random),
            ridges: RIDGE.init(random),
            jagged: JAGGED.init(random),
        };
        let chunk_height = ChunkHeight {
            min_y: -64,
            height: 384,
        };
        Self {
            seed,
            chunk_height,
            biome_noise,
            biomes: overworld_biomes(),
            surface: OverworldSurface::new(rng_unwrapped, chunk_height),
            carver: OverworldCarver::new(),
        }
    }

    fn generate_biomes(&self, pos: ChunkPos) -> BiomeChunk {
        BiomeChunk::generate(&self.biome_noise, &self.biomes, pos, self.chunk_height)
    }

    pub fn generate_chunk(&self, pos: ChunkPos) -> Chunk {
        let mut chunk = Chunk::new(pos.pos.x, pos.pos.y, "overworld".to_string());
        chunk
    }
}
