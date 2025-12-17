use crate::{
    common::math::lerp3,
    random::{LegacyRandom, Rng},
};
use std::{array::from_fn, f64::consts::SQRT_3};

use bevy_math::{DVec2, DVec3, FloatExt, IVec3};
use const_str::starts_with;

//reference net.minecraft.world.level.levelgen.Noises
//only shift and swamp have a different name in the resourcelocation, but we could rename them and
//do some macro magic (maybe)
//TODO: move
pub const NETHER_TEMPERATURE: ConstNormalNoise<2> =
    ConstNormalNoise::new("minecraft:temperature", -7, [1.0, 1.0]);
pub const NETHER_VEGETATION: ConstNormalNoise<2> =
    ConstNormalNoise::new("minecraft:vegetation", -7, [1.0, 1.0]);
pub const NETHERRACK: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:netherrack", -3, [1.0, 0.0, 0.0, 0.35]);
pub const NETHER_WART: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:nether_wart", -3, [1.0, 0.0, 0.0, 0.9]);
const SOUL_SAND_LAYER: ConstNormalNoise<9> = ConstNormalNoise::new(
    "minecraft:soul_sand_layer",
    -8,
    [1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.013333333333333334],
);
const GRAVEL_LAYER: ConstNormalNoise<9> = ConstNormalNoise::new(
    "minecraft:gravel_layer",
    -8,
    [1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.013333333333333334],
);
const PATCH: ConstNormalNoise<6> = ConstNormalNoise::new(
    "minecraft:patch",
    -5,
    [1.0, 0.0, 0.0, 0.0, 0.0, 0.013333333333333334],
);
pub const NETHER_STATE_SELECTOR: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:nether_state_selector", -4, [1.0]);
pub const BASE_3D_NOISE_NETHER: ConstBlendedNoise =
    ConstBlendedNoise::new(0.375 * 684.412 * 8., DVec3::new(80.0, 60.0, 80.0));

pub struct ConstBlendedNoise {
    min_limit_noise: ConstPerlinNoise<16>,
    max_limit_noise: ConstPerlinNoise<16>,
    main_noise: ConstPerlinNoise<8>,
    y_scale: f64,
    factor: DVec3,
}
impl ConstBlendedNoise {
    pub const fn new(y_scale: f64, factor: DVec3) -> Self {
        Self {
            min_limit_noise: ConstPerlinNoise::new(-15, [1.0; 16]),
            max_limit_noise: ConstPerlinNoise::new(-15, [1.0; 16]),
            main_noise: ConstPerlinNoise::new(-7, [1.0; 8]),
            y_scale,
            factor,
        }
    }
    pub fn init(&self, random: &mut impl Rng) -> BlendedNoise {
        BlendedNoise {
            min_limit_noise: self.min_limit_noise.legacy_init(random),
            max_limit_noise: self.max_limit_noise.legacy_init(random),
            main_noise: self.main_noise.legacy_init(random),
            y_scale: self.y_scale,
            factor: self.factor,
        }
    }
}

pub struct BlendedNoise {
    min_limit_noise: PerlinNoise<16>,
    max_limit_noise: PerlinNoise<16>,
    main_noise: PerlinNoise<8>,
    y_scale: f64,
    factor: DVec3,
}
impl BlendedNoise {
    pub fn at(&self, pos: DVec3) -> f64 {
        let main = self
            .main_noise
            .legacy_blended_at(pos / self.factor, self.y_scale / self.factor.y)
            / 20.
            + 0.5;
        let min = self.min_limit_noise.legacy_blended_at(pos, self.y_scale);
        let max = self.max_limit_noise.legacy_blended_at(pos, self.y_scale);
        min.lerp(max, main.clamp(0.0, 1.0)) / 65536.0
    }
}
pub struct ConstNormalNoise<const N: usize> {
    first: ConstPerlinNoise<N>,
    second: ConstPerlinNoise<N>,
    factor: f64,
    name: &'static str,
}

impl<const N: usize> ConstNormalNoise<N> {
    pub const fn new(name: &'static str, first_octave: i32, amplitudes: [f64; N]) -> Self {
        assert!(first_octave <= 0);
        assert!(amplitudes[0] != 0.0);
        assert!(starts_with!(name, "minecraft:"));

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
            factor: 5. / 3. - 5. / (3 * last_idx + 6) as f64,
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
    first_octave: i32,
    amplitudes: [f64; N],
}

impl<const N: usize> ConstPerlinNoise<N> {
    pub const fn new(first_octave: i32, amplitudes: [f64; N]) -> Self {
        assert!(!amplitudes.is_empty());
        assert!(first_octave <= 0);
        Self {
            first_octave,
            amplitudes,
        }
    }

    pub fn legacy_init(&self, random: &mut impl Rng) -> PerlinNoise<N> {
        PerlinNoise {
            noise_levels: from_fn(|_| ImprovedNoise::new(random)),
            amplitudes: self.amplitudes,
            first_octave: self.first_octave,
        }
    }

    pub fn init(&self, factory: impl Rng) -> PerlinNoise<N> {
        PerlinNoise {
            noise_levels: from_fn(|i| {
                ImprovedNoise::new(
                    &mut factory.with_hash(&format!("octave_{}", i as i32 + self.first_octave)),
                )
            }),
            amplitudes: self.amplitudes,
            first_octave: self.first_octave,
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
    pub fn at(&self, pos: impl Into<DVec3>) -> f64 {
        let pos = pos.into();
        (self.first.at(pos) + self.second.at(pos * 1.0181268882175227)) * self.factor
    }
}

///reference: net.minecraft.world.level.levelgen.synth.PerlinNoise
pub struct PerlinNoise<const N: usize> {
    noise_levels: [ImprovedNoise; N],
    amplitudes: [f64; N],
    first_octave: i32,
}

impl<const N: usize> PerlinNoise<N> {
    pub fn at(&self, point: DVec3) -> f64 {
        self.noise_levels
            .iter()
            .zip(self.amplitudes)
            .enumerate()
            .map(|(i, (noise, amp))| {
                amp * f64::from(1 << (N - 1 - i)) / f64::from((1 << N) - 1)
                    * noise.at((point * 2f64.powi(self.first_octave + i as i32)).map(wrap))
            })
            .sum()
    }

    fn legacy_blended_at(&self, point: DVec3, y_scale: f64) -> f64 {
        let mut res = 0.0;
        let mut scale = 1.0;

        //assume all amps are 1.
        for noise in &self.noise_levels {
            res += noise.legacy_at((point * scale).map(wrap), y_scale * scale, point.y * scale)
                / scale;
            scale /= 2.0;
        }

        res
    }

    pub fn legacy_simplex_at(&self, point: DVec2) -> f64 {
        let point = point * 2f64.powi(self.first_octave);
        let mut res = 0.0;
        let mut scale = 1.;

        for (noise, amp) in self.noise_levels.iter().zip(self.amplitudes) {
            res += amp * noise.legacy_simplex_at(point * scale) / scale;
            scale *= 2.0;
        }

        res
    }
}

fn smoothstep(input: f64) -> f64 {
    input * input * input * (input * (input * 6.0 - 15.0) + 10.0)
}

fn wrap(input: f64) -> f64 {
    const ROUND_OFF: f64 = 2i32.pow(25) as f64;
    input - ((input / ROUND_OFF).round() * ROUND_OFF)
}

/// reference: net.minecraft.world.level.levelgen.synth.ImprovedNoise
pub struct ImprovedNoise {
    p: [u8; 256],
    offset: DVec3,
}

impl ImprovedNoise {
    pub fn new(random: &mut impl Rng) -> Self {
        let offset = DVec3::new(random.next_f64(), random.next_f64(), random.next_f64()) * 256.0;
        let mut p = from_fn(|i| i as u8);
        random.shuffle(&mut p);
        Self { p, offset }
    }

    fn corner_noise(&self, index: i32, point: DVec2) -> f64 {
        const SIMPLEX_GRADIENT: [DVec2; 12] = [
            DVec2::new(1.0, 1.0),
            DVec2::new(-1.0, 1.0),
            DVec2::new(1.0, -1.0),
            DVec2::new(-1.0, -1.0),
            DVec2::new(1.0, 0.0),
            DVec2::new(-1.0, 0.0),
            DVec2::new(1.0, 0.0),
            DVec2::new(-1.0, 0.0),
            DVec2::new(0.0, 1.0),
            DVec2::new(0.0, -1.0),
            DVec2::new(0.0, 1.0),
            DVec2::new(0.0, -1.0),
        ];
        (0.5 - point.length_squared()).max(0.0).powi(4)
            * point.dot(SIMPLEX_GRADIENT[(self.p(index) % 12) as usize])
    }

    pub fn legacy_simplex_at(&self, point: DVec2) -> f64 {
        const F2: f64 = 0.5 * (SQRT_3 - 1.0);
        const G2: f64 = (3.0 - SQRT_3) / 6.0;

        let grid = (point + point.element_sum() * F2).floor().as_ivec2();

        let grid_d = grid.as_dvec2();
        // removing these braces lets parity tests fail.
        let delta_1 = point - (grid_d - grid_d.element_sum() * G2);

        let sub = if delta_1.x > delta_1.y {
            DVec2::new(1., 0.)
        } else {
            DVec2::new(0., 1.)
        };

        let delta_2 = delta_1 - sub + G2;
        let delta_3 = delta_1 - 1.0 + 2.0 * G2;
        let corner_noise3d = self.corner_noise(grid.x + self.p(grid.y), delta_1);
        let corner_noise3d1 = self.corner_noise(
            grid.x + sub.x as i32 + self.p(grid.y + sub.y as i32),
            delta_2,
        );
        let corner_noise3d2 = self.corner_noise(grid.x + 1 + self.p(grid.y + 1), delta_3);
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
    fn gradient(&self, delta: DVec3, grid: IVec3) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
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
        p.dot(SIMPLEX_GRADIENT[self.p(index) as usize & 15])
    }
}

#[test]
fn test_normal_noise() {
    let rng = crate::random::Xoroshiro128PlusPlus::new(0, 0).fork();
    let noise =
        ConstNormalNoise::new("minecraft:test", -4, [2.0, 1.5, 0.1, -1.0, 0.0, 0.0]).init(rng);

    //TODO: vanilla has slight rounding errors here so the result is instead 0.3623879633162622
    assert_eq!(
        noise.at(DVec3::new(0.0, 0.0, 0.0)),
        0.3623879633162623,
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

    let perlin_noise = ConstPerlinNoise::new(-2, [1.0, -1.0, 0.0, 0.5, 0.0]).init(rng);

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
    let noise =
        ConstBlendedNoise::new(0.125 * 684.412 * 8., DVec3::new(80.0, 160.0, 80.0)).init(&mut rng);
    assert_eq!(noise.at(DVec3::new(0.0, 0.0, 0.0)), 0.05283812245734512);
    assert_eq!(
        noise.at(DVec3::new(10000.0, 203.0, -20031.0) * DVec3::new(1.0, 0.125, 1.0) * 684.412),
        -0.021018525929896836
    );
}

#[test]
fn test_simplex_noise() {
    let mut rng = crate::random::Xoroshiro128PlusPlus::from_seed(0);
    let noise = ImprovedNoise::new(&mut rng);

    assert_eq!(
        noise.legacy_simplex_at(DVec2::new(0.0, 0.0)),
        0.0,
        "Mismatch at zero"
    );
    // TODO: the vanilla result is 0.16818932411152746, but due to floating point
    // inaccuracies, we are getting a slightly different result. Fine for now.
    assert_eq!(
        noise.legacy_simplex_at(DVec2::new(10000.0, -20031.0)),
        0.16818932411152765
    );
}
