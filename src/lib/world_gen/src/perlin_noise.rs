use crate::random::{LegacyRandom, Rng};
use std::{array::from_fn, f64::consts::SQRT_3, sync::LazyLock};

use bevy_math::{DVec2, DVec3, FloatExt, Vec3Swizzles};

//reference net.minecraft.world.level.levelgen.Noises
//only shift and swamp have a different name in the resourcelocation, but we could rename them and
//do some macro magic (maybe)
//TODO: move
pub const NETHER_TEMPERATURE: ConstNormalNoise<2> =
    ConstNormalNoise::new("minecraft:temperature", 7, [1.0, 1.0]);
pub const NETHER_VEGETATION: ConstNormalNoise<2> =
    ConstNormalNoise::new("minecraft:vegetation", 7, [1.0, 1.0]);
pub const RIDGE: ConstNormalNoise<6> =
    ConstNormalNoise::new("minecraft:ridge", 7, [1.0, 2.0, 1.0, 0.0, 0.0, 0.0]);
pub const SHIFT: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:offset", 3, [1.0, 1.0, 1.0, 0.0]);
pub const TEMPERATURE: ConstNormalNoise<6> =
    ConstNormalNoise::new("minecraft:temperature", 10, [1.5, 0.0, 1.0, 0.0, 0.0, 0.0]);
pub const VEGETATION: ConstNormalNoise<6> =
    ConstNormalNoise::new("minecraft:vegetation", 8, [1.0, 1.0, 0.0, 0.0, 0.0, 0.0]);
pub const CONTINENTALNESS: ConstNormalNoise<9> = ConstNormalNoise::new(
    "minecraft:continentalness",
    9,
    [1.0, 1.0, 2.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0],
);
pub const EROSION: ConstNormalNoise<5> =
    ConstNormalNoise::new("minecraft:erosion", 9, [1.0, 1.0, 0.0, 1.0, 1.0]);
pub const AQUIFER_BARRIER: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:aquifer_barrier", 3, [1.0]);
pub const AQUIFER_FLUID_LEVEL_FLOODEDNESS: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:aquifer_fluid_level_floodedness", 7, [1.0]);
pub const AQUIFER_LAVA: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:aquifer_lava", 1, [1.0]);
pub const AQUIFER_FLUID_LEVEL_SPREAD: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:aquifer_fluid_level_spread", 5, [1.0]);
pub const PILLAR: ConstNormalNoise<2> = ConstNormalNoise::new("minecraft:pillar", 7, [1.0, 1.0]);
pub const PILLAR_RARENESS: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:pillar_rareness", 8, [1.0]);
pub const PILLAR_THICKNESS: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:pillar_thickness", 8, [1.0]);
pub const SPAGHETTI_2D: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:spaghetti_2d", 7, [1.0]);
pub const SPAGHETTI_2D_ELEVATION: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:spaghetti_2d_elevation", 8, [1.0]);
pub const SPAGHETTI_2D_MODULATOR: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:spaghetti_2d_modulator", 11, [1.0]);
pub const SPAGHETTI_2D_THICKNESS: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:spaghetti_2d_thickness", 11, [1.0]);
pub const SPAGHETTI_3D_1: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:spaghetti_3d_1", 7, [1.0]);
pub const SPAGHETTI_3D_2: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:spaghetti_3d_2", 7, [1.0]);
pub const SPAGHETTI_3D_RARITY: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:spaghetti_3d_rarity", 11, [1.0]);
pub const SPAGHETTI_3D_THICKNESS: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:spaghetti_3d_thickness", 8, [1.0]);
pub const SPAGHETTI_ROUGHNESS: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:spaghetti_roughness", 5, [1.0]);
pub const SPAGHETTI_ROUGHNESS_MODULATOR: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:spaghetti_roughness_modulator", 8, [1.0]);
pub const CAVE_ENTRANCE: ConstNormalNoise<3> =
    ConstNormalNoise::new("minecraft:cave_entrance", 7, [0.4, 0.5, 1.0]);
pub const CAVE_LAYER: ConstNormalNoise<1> = ConstNormalNoise::new("minecraft:cave_layer", 8, [1.0]);
pub const CAVE_CHEESE: ConstNormalNoise<9> = ConstNormalNoise::new(
    "minecraft:cave_cheese",
    8,
    [0.5, 1.0, 2.0, 1.0, 2.0, 1.0, 0.0, 2.0, 0.0],
);
pub const ORE_VEININESS: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:ore_veininess", 8, [1.0]);
pub const ORE_VEIN_A: ConstNormalNoise<1> = ConstNormalNoise::new("minecraft:ore_vein_a", 7, [1.0]);
pub const ORE_VEIN_B: ConstNormalNoise<1> = ConstNormalNoise::new("minecraft:ore_vein_b", 7, [1.0]);
pub const ORE_GAP: ConstNormalNoise<1> = ConstNormalNoise::new("minecraft:ore_gap", 5, [1.0]);
pub const NOODLE: ConstNormalNoise<1> = ConstNormalNoise::new("minecraft:noodle", 8, [1.0]);
pub const NOODLE_THICKNESS: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:noodle_thickness", 8, [1.0]);
pub const NOODLE_RIDGE_A: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:noodle_ridge_a", 7, [1.0]);
pub const NOODLE_RIDGE_B: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:noodle_ridge_b", 7, [1.0]);
pub const JAGGED: ConstNormalNoise<16> = ConstNormalNoise::new(
    "minecraft:jagged",
    16,
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    ],
);
pub const SURFACE: ConstNormalNoise<3> =
    ConstNormalNoise::new("minecraft:surface", 6, [1.0, 1.0, 1.0]);
pub const SURFACE_SECONDARY: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:surface_secondary", 6, [1.0, 1.0, 0.0, 1.0]);
pub const CLAY_BANDS_OFFSET: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:clay_bands_offset", 8, [1.0]);
pub const BADLANDS_PILLAR: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:badlands_pillar", 2, [1.0, 1.0, 1.0, 1.0]);
pub const BADLANDS_PILLAR_ROOF: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:badlands_pillar_roof", 8, [1.0]);
pub const BADLANDS_SURFACE: ConstNormalNoise<3> =
    ConstNormalNoise::new("minecraft:badlands_surface", 6, [1.0, 1.0, 1.0]);
pub const ICEBERG_PILLAR: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:iceberg_pillar", 6, [1.0, 1.0, 1.0, 1.0]);
pub const ICEBERG_PILLAR_ROOF: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:iceberg_pillar_roof", 3, [1.0]);
pub const ICEBERG_SURFACE: ConstNormalNoise<3> =
    ConstNormalNoise::new("minecraft:iceberg_surface", 6, [1.0, 1.0, 1.0]);
pub const SWAMP: ConstNormalNoise<1> = ConstNormalNoise::new("minecraft:surface_swamp", 2, [1.0]);
pub const CALCITE: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:calcite", 9, [1.0, 1.0, 1.0, 1.0]);
pub const GRAVEL: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:gravel", 8, [1.0, 1.0, 1.0, 1.0]);
pub const POWDER_SNOW: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:powder_snow", 6, [1.0, 1.0, 1.0, 1.0]);
pub const PACKED_ICE: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:packed_ice", 7, [1.0, 1.0, 1.0, 1.0]);
pub const ICE: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:ice", 4, [1.0, 1.0, 1.0, 1.0]);
pub const SOUL_SAND_LAYER: ConstNormalNoise<9> = ConstNormalNoise::new(
    "minecraft:soul_sand_layer",
    8,
    [1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.013333333333333334],
);
pub const GRAVEL_LAYER: ConstNormalNoise<9> = ConstNormalNoise::new(
    "minecraft:gravel_layer",
    8,
    [1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.013333333333333334],
);
pub const PATCH: ConstNormalNoise<6> = ConstNormalNoise::new(
    "minecraft:patch",
    5,
    [1.0, 0.0, 0.0, 0.0, 0.0, 0.013333333333333334],
);
pub const NETHERRACK: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:netherrack", 3, [1.0, 0.0, 0.0, 0.35]);
pub const NETHER_WART: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:nether_wart", 3, [1.0, 0.0, 0.0, 0.9]);
pub const NETHER_STATE_SELECTOR: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:nether_state_selector", 4, [1.0]);
pub const BASE_3D_NOISE_OVERWORLD: ConstBlendedNoise =
    ConstBlendedNoise::new(0.125, DVec3::new(80.0, 160.0, 80.0), 8.0);
pub static BIOME_INFO_NOISE: LazyLock<PerlinNoise<1>> =
    LazyLock::new(|| ConstPerlinNoise::new(0, [1.0]).legacy_init(&mut LegacyRandom::new(2345))); //TODO:
//make const
pub static TEMPERATURE_NOISE: LazyLock<PerlinNoise<1>> =
    LazyLock::new(|| ConstPerlinNoise::new(0, [1.0]).legacy_init(&mut LegacyRandom::new(1234))); //TODO:
//make const
pub static FROZEN_TEMPERATURE_NOISE: LazyLock<PerlinNoise<3>> = LazyLock::new(|| {
    ConstPerlinNoise::new(2, [1.0, 1.0, 1.0]).legacy_init(&mut LegacyRandom::new(3456))
}); //TODO:
//make const

pub struct ConstBlendedNoise {
    min_limit_noise: ConstPerlinNoise<16>,
    max_limit_noise: ConstPerlinNoise<16>,
    main_noise: ConstPerlinNoise<8>,
    y_scale: f64,
    factor: DVec3,
    smear_scale_multiplier: f64,
}
impl ConstBlendedNoise {
    const fn new(y_scale: f64, factor: DVec3, smear_scale_multiplier: f64) -> Self {
        Self {
            min_limit_noise: ConstPerlinNoise::new(15, [1.0; 16]),
            max_limit_noise: ConstPerlinNoise::new(15, [1.0; 16]),
            main_noise: ConstPerlinNoise::new(7, [1.0; 8]),
            y_scale,
            factor,
            smear_scale_multiplier,
        }
    }
    pub fn init(&self, random: &mut impl Rng) -> BlendedNoise {
        BlendedNoise {
            min_limit_noise: self.min_limit_noise.legacy_init(random),
            max_limit_noise: self.max_limit_noise.legacy_init(random),
            main_noise: self.main_noise.legacy_init(random),
            y_scale: self.y_scale,
            factor: self.factor,
            smear_scale_multiplier: self.smear_scale_multiplier,
        }
    }
}

pub struct BlendedNoise {
    min_limit_noise: PerlinNoise<16>,
    max_limit_noise: PerlinNoise<16>,
    main_noise: PerlinNoise<8>,
    y_scale: f64,
    factor: DVec3,
    smear_scale_multiplier: f64,
}
impl BlendedNoise {
    pub fn at(&self, pos: DVec3) -> f64 {
        let scale = self.y_scale * 684.412 * self.smear_scale_multiplier;

        let main_noise = self
            .main_noise
            .legacy_blended_at(pos / self.factor, scale / self.factor.y);

        let main = (main_noise / 10.0 + 1.0) / 2.0;
        let min = if main < 1.0 {
            self.min_limit_noise.legacy_blended_at(pos, scale)
        } else {
            0.0
        };
        let max = if main > 0.0 {
            self.max_limit_noise.legacy_blended_at(pos, scale)
        } else {
            0.0
        };

        (min / 512.0).lerp(max / 512.0, main.clamp(0.0, 1.0)) / 128.0
    }
}
pub struct ConstNormalNoise<const N: usize> {
    first: ConstPerlinNoise<N>,
    second: ConstPerlinNoise<N>,
    factor: f64,
    name: &'static str,
}

impl<const N: usize> ConstNormalNoise<N> {
    pub const fn new(name: &'static str, first_octave: u32, amplitudes: [f64; N]) -> Self {
        assert!(amplitudes[0] != 0.0);
        const fn str_start_with(s: &str, prefix: &str) -> bool {
            let bytes = s.as_bytes();
            let prefix_bytes = prefix.as_bytes();

            if bytes.len() < prefix_bytes.len() {
                return false;
            }

            let mut i = 0;
            while i < prefix_bytes.len() {
                if bytes[i] != prefix_bytes[i] {
                    return false;
                }
                i += 1;
            }

            true
        }
        assert!(str_start_with(name, "minecraft:"));

        let mut last_idx = 0;
        let mut i = N;
        while i > 0 {
            i -= 1;
            if amplitudes[i] != 0.0 {
                last_idx = i;
                break;
            }
        }
        Self {
            name,
            factor: 1.0 / (0.6 + 6.0 / (10 * (last_idx + 1)) as f64),
            first: ConstPerlinNoise::new(first_octave, amplitudes),
            second: ConstPerlinNoise::new(first_octave, amplitudes),
        }
    }

    pub fn legacy_init(&self, seed: u64) -> NormalNoise<N> {
        let mut random = LegacyRandom::new(seed);
        NormalNoise {
            first: self.first.legacy_init(&mut random),
            second: self.second.legacy_init(&mut random),
            factor: self.factor,
        }
    }
    pub fn init(&self, factory: impl Rng) -> NormalNoise<N> {
        let mut rng = factory.with_hash(self.name);
        NormalNoise {
            first: self.first.init(rng.fork()),
            second: self.second.init(rng.fork()),
            factor: self.factor,
        }
    }
}

pub struct ConstPerlinNoise<const N: usize> {
    first_octave: u32,
    amplitudes: [f64; N],
    lowest_freq_input_factor: f64,
    lowest_freq_value_factor: f64,
}

impl<const N: usize> ConstPerlinNoise<N> {
    pub const fn new(first_octave: u32, amplitudes: [f64; N]) -> Self {
        assert!(!amplitudes.is_empty());
        let lowest_freq_input_factor = 1.0 / 2i32.pow(first_octave) as f64;
        let lowest_freq_value_factor =
            2u32.pow(N as u32 - 1) as f64 / (2u32.pow(N as u32) - 1) as f64;

        Self {
            first_octave,
            amplitudes,
            lowest_freq_input_factor,
            lowest_freq_value_factor,
        }
    }

    pub fn legacy_init(&self, random: &mut impl Rng) -> PerlinNoise<N> {
        PerlinNoise {
            noise_levels: from_fn(|_| ImprovedNoise::new(random)),
            amplitudes: self.amplitudes,
            lowest_freq_input_factor: self.lowest_freq_input_factor,
            lowest_freq_value_factor: self.lowest_freq_value_factor,
        }
    }

    pub fn init(&self, factory: impl Rng) -> PerlinNoise<N> {
        PerlinNoise {
            noise_levels: from_fn(|i| {
                ImprovedNoise::new(
                    &mut factory
                        .with_hash(&format!("octave_{}", i as i32 - self.first_octave as i32)),
                )
            }),
            amplitudes: self.amplitudes,
            lowest_freq_input_factor: self.lowest_freq_input_factor,
            lowest_freq_value_factor: self.lowest_freq_value_factor,
        }
    }
}

///reference: net.minecraft.world.level.levelgen.synth.NormalNoise
pub struct NormalNoise<const N: usize> {
    first: PerlinNoise<N>,
    second: PerlinNoise<N>,
    factor: f64,
}

impl<const N: usize> NormalNoise<N> {
    pub fn at(&self, pos: DVec3) -> f64 {
        (self.first.at(pos) + self.second.at(pos * 1.0181268882175227)) * self.factor
    }
}

///reference: net.minecraft.world.level.levelgen.synth.PerlinNoise
#[derive(Debug)]
pub struct PerlinNoise<const N: usize> {
    noise_levels: [ImprovedNoise; N],
    amplitudes: [f64; N],
    lowest_freq_input_factor: f64,
    lowest_freq_value_factor: f64,
}

impl<const N: usize> PerlinNoise<N> {
    pub fn at(&self, point: DVec3) -> f64 {
        let mut res = 0.0;
        let mut freq_input_factor = self.lowest_freq_input_factor;
        let mut freq_value_factor = self.lowest_freq_value_factor;

        for (noise, amp) in self.noise_levels.iter().zip(self.amplitudes) {
            res += amp * noise.at((point * freq_input_factor).map(wrap)) * freq_value_factor;

            freq_input_factor *= 2.0;
            freq_value_factor /= 2.0;
        }

        res
    }

    fn legacy_blended_at(&self, point: DVec3, y_scale: f64) -> f64 {
        let mut res = 0.0;
        let mut weight = 1.0;

        for noise in &self.noise_levels {
            res += noise.legacy_at(
                (point * weight).map(wrap),
                y_scale * weight,
                point.y * weight,
            ) / weight;
            weight /= 2.0;
        }

        res
    }

    pub fn legacy_simplex_at(&self, point: DVec2) -> f64 {
        let mut res = 0.0;
        let mut freq_input_factor = self.lowest_freq_input_factor;
        let mut freq_value_factor = self.lowest_freq_value_factor;

        for (noise, amp) in self.noise_levels.iter().zip(self.amplitudes) {
            res += amp * noise.legacy_simplex_at(point * freq_input_factor) * freq_value_factor;

            freq_input_factor *= 2.0;
            freq_value_factor /= 2.0;
        }

        res
    }
}

#[inline]
fn smoothstep(input: f64) -> f64 {
    input * input * input * (input * (input * 6.0 - 15.0) + 10.0)
}

#[inline]
fn wrap(input: f64) -> f64 {
    input - ((input / 2f64.powi(25) + 0.5).floor() * 2f64.powi(25))
}

/// reference: net.minecraft.world.level.levelgen.synth.ImprovedNoise
#[derive(Debug)]
pub struct ImprovedNoise {
    p: [u8; 256],
    offset: DVec3,
}

impl ImprovedNoise {
    #[inline]
    fn new(random: &mut impl Rng) -> Self {
        let offset = DVec3::new(random.next_f64(), random.next_f64(), random.next_f64()) * 256.0;

        let mut p = from_fn(|i| i as u8);

        random.shuffle(&mut p);

        Self { p, offset }
    }

    #[inline]
    fn corner_noise(&self, index: i32, point: DVec3, offset: f64) -> f64 {
        (offset - point.length_squared()).max(0.0).powi(4) * self.grad_dot(index % 12, point)
    }
    #[inline]
    fn legacy_simplex_at(&self, at: DVec2) -> f64 {
        const F2: f64 = 0.5 * (SQRT_3 - 1.0);
        const G2: f64 = (3.0 - SQRT_3) / 6.0;

        let d = (at.x + at.y) * F2;
        let floor = at.x + d;
        let floor1 = at.y + d;
        let i_floor = floor.floor() as i32;
        let i_floor1 = floor1.floor() as i32;

        let d1 = f64::from(i_floor + i_floor1) * G2;
        let d2 = f64::from(i_floor) - d1;
        let d3 = f64::from(i_floor1) - d1;
        let d4 = at.x - d2;
        let d5 = at.y - d3;

        let (i, i1) = if d4 > d5 { (1, 0) } else { (0, 1) };

        let d6 = d4 - f64::from(i) + G2;
        let d7 = d5 - f64::from(i1) + G2;
        let d8 = d4 - 1.0 + 2.0 * G2;
        let d9 = d5 - 1.0 + 2.0 * G2;

        let i2 = i_floor & 0xFF;
        let i3 = i_floor1 & 0xFF;

        let corner_noise3d = self.corner_noise(i2 + self.p(i_floor1), DVec3::new(d4, d5, 0.0), 0.5);
        let corner_noise3d1 =
            self.corner_noise(i2 + i + self.p(i3 + i1), DVec3::new(d6, d7, 0.0), 0.5);
        let corner_noise3d2 =
            self.corner_noise(i2 + 1 + self.p(i3 + 1), DVec3::new(d8, d9, 0.0), 0.5);

        70.0 * (corner_noise3d + corner_noise3d1 + corner_noise3d2)
    }

    fn legacy_at(&self, at: DVec3, y_scale: f64, y_max: f64) -> f64 {
        assert!(y_scale != 0.0);
        let actual = at + self.offset;
        let grid = actual.floor();
        let delta = actual - grid;

        let grid = grid.as_ivec3();
        let weird_delta =
            delta.with_y(delta.y - (delta.y.min(y_max) / y_scale + 1.0E-7).floor() * y_scale);
        let (d, d1, d2, d3, d4, d5, d6, d7) = self.gradient(weird_delta, grid);

        let smooth = delta.map(smoothstep);

        lerp3(smooth, d, d1, d2, d3, d4, d5, d6, d7)
    }

    pub fn at(&self, at: DVec3) -> f64 {
        let actual = at + self.offset;
        let grid = actual.floor();
        let delta = actual - grid;

        let grid = grid.as_ivec3();
        let (d, d1, d2, d3, d4, d5, d6, d7) = self.gradient(delta, grid);

        let smooth = delta.map(smoothstep);

        lerp3(smooth, d, d1, d2, d3, d4, d5, d6, d7)
    }

    #[inline]
    fn gradient(
        &self,
        delta: DVec3,
        grid: bevy_math::IVec3,
    ) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let x = self.p(grid.x);
        let x1 = self.p(grid.x + 1);
        let y = self.p(x + grid.y);
        let y1 = self.p(x + grid.y + 1);
        let x1y = self.p(x1 + grid.y);
        let x1y1 = self.p(x1 + grid.y + 1);

        let d = self.grad_dot(y + grid.z, delta);
        let d1 = self.grad_dot(x1y + grid.z, delta - DVec3::new(1.0, 0.0, 0.0));
        let d2 = self.grad_dot(y1 + grid.z, delta - DVec3::new(0.0, 1.0, 0.0));
        let d3 = self.grad_dot(x1y1 + grid.z, delta - DVec3::new(1.0, 1.0, 0.0));
        let d4 = self.grad_dot(y + grid.z + 1, delta - DVec3::new(0.0, 0.0, 1.0));
        let d5 = self.grad_dot(x1y + grid.z + 1, delta - DVec3::new(1.0, 0.0, 1.0));
        let d6 = self.grad_dot(y1 + grid.z + 1, delta - DVec3::new(0.0, 1.0, 1.0));
        let d7 = self.grad_dot(x1y1 + grid.z + 1, delta - DVec3::new(1.0, 1.0, 1.0));
        (d, d1, d2, d3, d4, d5, d6, d7)
    }

    #[inline]
    const fn p(&self, index: i32) -> i32 {
        self.p[(index & 0xFF) as usize] as i32
    }

    #[inline]
    fn grad_dot(&self, index: i32, p: DVec3) -> f64 {
        let grad_index = self.p(index);
        const SIMPLEX_GRADIENT: [DVec3; 16] = [
            DVec3::new(1.0, 1.0, 0.0),
            DVec3::new(-1.0, 1.0, 0.0),
            DVec3::new(1.0, -1.0, 0.0),
            DVec3::new(-1.0, -1.0, 0.0),
            DVec3::new(1.0, 0.0, 1.0),
            DVec3::new(-1.0, 0.0, 1.0),
            DVec3::new(1.0, 0.0, -1.0),
            DVec3::new(-1.0, 0.0, -1.0),
            DVec3::new(0.0, 1.0, 1.0),
            DVec3::new(0.0, -1.0, 1.0),
            DVec3::new(0.0, 1.0, -1.0),
            DVec3::new(0.0, -1.0, -1.0),
            DVec3::new(1.0, 1.0, 0.0),
            DVec3::new(0.0, -1.0, 1.0),
            DVec3::new(-1.0, 1.0, 0.0),
            DVec3::new(0.0, -1.0, -1.0),
        ];
        p.dot(SIMPLEX_GRADIENT[grad_index as usize & 15])
    }
}

pub fn lerp2(delta: DVec2, start1: f64, end1: f64, start2: f64, end2: f64) -> f64 {
    start1
        .lerp(end1, delta.x)
        .lerp(start2.lerp(end2, delta.x), delta.y)
}

#[allow(clippy::too_many_arguments)]
pub fn lerp3(
    delta: DVec3,
    start1: f64,
    end1: f64,
    start2: f64,
    end2: f64,
    start3: f64,
    end3: f64,
    start4: f64,
    end4: f64,
) -> f64 {
    lerp2(delta.xy(), start1, end1, start2, end2)
        .lerp(lerp2(delta.xy(), start3, end3, start4, end4), delta.z)
}

#[test]
fn test_normal_noise() {
    let rng = crate::random::Xoroshiro128PlusPlus::new(0, 0).fork();
    let noise =
        ConstNormalNoise::new("minecraft:test", 4, [2.0, 1.5, 0.1, -1.0, 0.0, 0.0]).init(rng);

    assert_eq!(
        noise.at(DVec3::new(0.0, 0.0, 0.0)),
        0.3623879633162622,
        "Mismatch in noise at zero"
    );
    assert_eq!(
        noise.at(DVec3::new(10000.123, 203.5, -20031.78)),
        -0.10086538185785067,
        "Mismatch in noise"
    );
}

#[test]
fn test_improved_noise() {
    let mut rng = crate::random::Xoroshiro128PlusPlus::new(0, 0);
    let noise = ImprovedNoise::new(&mut rng);

    assert_eq!(
        noise.at(DVec3::new(0.0, 0.0, 0.0)),
        -0.045044799854318,
        "Mismatch in noise at zero"
    );
    assert_eq!(
        noise.at(DVec3::new(10000.123, 203.5, -20031.78)),
        -0.18708168179464396,
        "Mismatch in noise"
    );
    assert_eq!(
        noise.legacy_at(DVec3::new(10000.123, 203.5, -20031.78), 0.5, 0.8),
        -0.31263505222083193,
        "Mismatch in legacy noise"
    );
}

#[test]
fn test_perlin_noise() {
    let rng = crate::random::Xoroshiro128PlusPlus::new(0, 0).fork();

    let perlin_noise = ConstPerlinNoise::new(2, [1.0, -1.0, 0.0, 0.5, 0.0]).init(rng);

    assert_eq!(
        perlin_noise.lowest_freq_input_factor, 0.25,
        "Mismatch in lowest_freq_input_factor"
    );
    assert_eq!(
        perlin_noise.lowest_freq_value_factor, 0.5161290322580645,
        "Mismatch in lowest_freq_value_factor"
    );
    assert_eq!(
        perlin_noise.at(DVec3::new(0.0, 0.0, 0.0)),
        -0.05992145275521602,
        "Mismatch in get_value at zero"
    );
    assert_eq!(
        perlin_noise.at(DVec3::new(10000.123, 203.5, -20031.78)),
        0.04676137080548814,
        "Mismatch in get_value"
    );
}

#[test]
fn test_blended_noise() {
    let mut rng = crate::random::Xoroshiro128PlusPlus::from_seed(0);
    let noise = BASE_3D_NOISE_OVERWORLD.init(&mut rng);
    assert_eq!(noise.at(DVec3::new(0.0, 0.0, 0.0)), 0.05283812245734512);
    assert_eq!(
        noise.at(DVec3::new(10000.0, 203.0, -20031.0) * DVec3::new(1.0, 0.125, 1.0) * 684.412),
        -0.021018525929896836
    );
}
