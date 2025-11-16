use crate::biome_chunk::BiomeNoise;
use crate::common::math::clamped_map;
use crate::overworld::noise_biome_parameters::{
    DEPTH_DEEP_DARK_DRYNESS_THRESHOLD, EROSION_DEEP_DARK_DRYNESS_THRESHOLD,
};
use crate::perlin_noise::{
    BASE_3D_NOISE_OVERWORLD, BlendedNoise, CAVE_CHEESE, CAVE_ENTRANCE, CAVE_LAYER, CONTINENTALNESS,
    EROSION, JAGGED, NOODLE, NOODLE_RIDGE_A, NOODLE_RIDGE_B, NOODLE_THICKNESS, NormalNoise, PILLAR,
    PILLAR_RARENESS, PILLAR_THICKNESS, RIDGE, SHIFT, SPAGHETTI_2D, SPAGHETTI_2D_ELEVATION,
    SPAGHETTI_2D_MODULATOR, SPAGHETTI_2D_THICKNESS, SPAGHETTI_3D_1, SPAGHETTI_3D_2,
    SPAGHETTI_3D_RARITY, SPAGHETTI_3D_THICKNESS, SPAGHETTI_ROUGHNESS,
    SPAGHETTI_ROUGHNESS_MODULATOR, TEMPERATURE, VEGETATION,
};
use crate::pos::{BlockPos, ChunkHeight, ChunkPos, ColumnPos};
use crate::random::Xoroshiro128PlusPlus;
use bevy_math::{DVec3, FloatExt, IVec3, Vec3Swizzles};
use ferrumc_macros::block;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk_format::Chunk;

use crate::overworld::spline::{CubicSpline, SplinePoint, SplineType};

//TODO: const
fn build_erosion_offset_spline(
    f: f32,
    f1: f32,
    f2: f32,
    magnitude: f32,
    f3: f32,
    f4: f32,
    use_max_slope: bool,
) -> CubicSpline {
    let cubic = build_mountain_ridge_spline_with_points(0.6.lerp(1.5, magnitude), use_max_slope);
    let cubic1 = build_mountain_ridge_spline_with_points(0.6.lerp(1.0, magnitude), use_max_slope);
    let cubic2 = build_mountain_ridge_spline_with_points(magnitude, use_max_slope);
    let cubic3 = ridge_spline(
        f - 0.15,
        0.5 * magnitude,
        0.5.lerp(0.5, 0.5) * magnitude,
        0.5 * magnitude,
        0.6 * magnitude,
        0.5,
    );
    let cubic4 = ridge_spline(
        f,
        f3 * magnitude,
        f1 * magnitude,
        0.5 * magnitude,
        0.6 * magnitude,
        0.5,
    );
    let cubic5 = ridge_spline(f, f3, f3, f1, f2, 0.5);
    let cubic6 = ridge_spline(f, f3, f3, f1, f2, 0.5);
    let cubic7 = CubicSpline::new(
        SplineType::RidgesFolded,
        vec![
            SplinePoint::constant(-1.0, f, 0.0),
            SplinePoint::spline(-0.4, cubic5.clone()),
            SplinePoint::constant(0.0, f2 + 0.07, 0.0),
        ],
    );
    let mut points = vec![
        SplinePoint::spline(-0.85, cubic),
        SplinePoint::spline(-0.7, cubic1),
        SplinePoint::spline(-0.4, cubic2),
        SplinePoint::spline(-0.35, cubic3),
        SplinePoint::spline(-0.1, cubic4),
        SplinePoint::spline(0.2, cubic5),
    ];

    if use_max_slope {
        points.extend(vec![
            SplinePoint::spline(0.4, cubic6.clone()),
            SplinePoint::spline(0.45, cubic7.clone()),
            SplinePoint::spline(0.55, cubic7),
            SplinePoint::spline(0.58, cubic6),
        ]);
    }

    points.push(SplinePoint::spline(
        0.7,
        ridge_spline(-0.02, f4, f4, f1, f2, 0.0),
    ));

    CubicSpline::new(SplineType::Erosion, points)
}

fn ridge_spline(y1: f32, y2: f32, y3: f32, y4: f32, y5: f32, min_smoothing: f32) -> CubicSpline {
    let max = (0.5 * (y2 - y1)).max(min_smoothing);
    let f = 5.0 * (y3 - y2);

    CubicSpline::new(
        SplineType::RidgesFolded,
        vec![
            SplinePoint::constant(-1.0, y1, max),
            SplinePoint::constant(-0.4, y2, max.min(f)),
            SplinePoint::constant(0.0, y3, f),
            SplinePoint::constant(0.4, y4, 2.0 * (y4 - y3)),
            SplinePoint::constant(1.0, y5, 0.7 * (y5 - y4)),
        ],
    )
}
fn build_mountain_ridge_spline_with_points(magnitude: f32, use_max_slope: bool) -> CubicSpline {
    fn mountain_continentalness(height_factor: f32, magnitude: f32, cutoff_height: f32) -> f32 {
        let f2 = 1.0 - (1.0 - magnitude) * 0.5;
        let f3 = 0.5 * (1.0 - magnitude);
        let f4 = (height_factor + 1.17) * 0.46082947;
        let f5 = f4 * f2 - f3;
        if height_factor < cutoff_height {
            f5.max(-0.2222)
        } else {
            f5.max(0.0)
        }
    }

    fn calculate_mountain_ridge_zero_continentalness_point(input: f32) -> f32 {
        let f2 = 1.0 - (1.0 - input) * 0.5;
        let f3 = 0.5 * (1.0 - input);
        f3 / (0.46082947 * f2) - 1.17
    }
    let f2 = mountain_continentalness(-1.0, magnitude, -0.7);
    let f3 = 1.0;
    let f4 = mountain_continentalness(1.0, magnitude, -0.7);
    let f5 = calculate_mountain_ridge_zero_continentalness_point(magnitude);
    let f6 = -0.65;

    let mut points = Vec::new();

    if f6 < f5 && f5 < f3 {
        let f7 = mountain_continentalness(f6, magnitude, -0.7);
        let f8 = -0.75;
        let f9 = mountain_continentalness(f8, magnitude, -0.7);
        let f10 = slope(f2, f9, -1.0, f8);

        points.push(SplinePoint::constant(-1.0, f2, f10));
        points.push(SplinePoint::constant(f8, f9, 0.0));
        points.push(SplinePoint::constant(f6, f7, 0.0));

        let f11 = mountain_continentalness(f5, magnitude, -0.7);
        let f12 = slope(f11, f4, f5, 1.0);

        points.push(SplinePoint::constant(f5 - 0.01, f11, 0.0));
        points.push(SplinePoint::constant(f5, f11, f12));
        points.push(SplinePoint::constant(1.0, f4, f12));
    } else {
        let f7 = slope(f2, f4, -1.0, 1.0);

        if use_max_slope {
            points.push(SplinePoint::constant(-1.0, f2.max(0.2), 0.0));
            points.push(SplinePoint::constant(0.0, f2.lerp(f4, 0.5), f7));
        } else {
            points.push(SplinePoint::constant(-1.0, f2, f7));
        }

        points.push(SplinePoint::constant(1.0, f4, f7));
    }

    CubicSpline::new(SplineType::RidgesFolded, points)
}

fn slope(y1: f32, y2: f32, x1: f32, x2: f32) -> f32 {
    (y2 - y1) / (x2 - x1)
}
pub(super) fn get_offset_spline() -> CubicSpline {
    let cubic = build_erosion_offset_spline(-0.15, 0.0, 0.0, 0.1, 0.0, -0.03, false);
    let cubic1 = build_erosion_offset_spline(-0.1, 0.03, 0.1, 0.1, 0.01, -0.03, false);
    let cubic2 = build_erosion_offset_spline(-0.1, 0.03, 0.1, 0.7, 0.01, -0.03, true);
    let cubic3 = build_erosion_offset_spline(-0.05, 0.03, 0.1, 1.0, 0.01, 0.01, true);

    CubicSpline::new(
        SplineType::Continents,
        vec![
            SplinePoint::constant(-1.1, 0.044, 0.0),
            SplinePoint::constant(-1.02, -0.2222, 0.0),
            SplinePoint::constant(-0.51, -0.2222, 0.0),
            SplinePoint::constant(-0.44, -0.12, 0.0),
            SplinePoint::constant(-0.18, -0.12, 0.0),
            SplinePoint::spline(-0.16, cubic.clone()),
            SplinePoint::spline(-0.15, cubic),
            SplinePoint::spline(-0.10, cubic1),
            SplinePoint::spline(0.25, cubic2),
            SplinePoint::spline(1.0, cubic3),
        ],
    )
}

pub fn overworld_factor() -> CubicSpline {
    CubicSpline::new(
        SplineType::Continents,
        vec![
            SplinePoint::constant(-0.19, 3.95, 0.0),
            SplinePoint::spline(-0.15, get_erosion_factor(6.25, true)),
            SplinePoint::spline(-0.1, get_erosion_factor(5.47, true)),
            SplinePoint::spline(0.03, get_erosion_factor(5.08, true)),
            SplinePoint::spline(0.06, get_erosion_factor(4.69, false)),
        ],
    )
}

fn get_erosion_factor(value: f32, higher_values: bool) -> CubicSpline {
    let cubic_spline = CubicSpline::new(
        SplineType::Ridges,
        vec![
            SplinePoint::constant(-0.2, 6.3, 0.0),
            SplinePoint::constant(0.2, value, 0.0),
        ],
    );

    let mut points = vec![
        SplinePoint::spline(-0.6, cubic_spline.clone()),
        SplinePoint::spline(
            -0.5,
            CubicSpline::new(
                SplineType::Ridges,
                vec![
                    SplinePoint::constant(-0.05, 6.3, 0.0),
                    SplinePoint::constant(0.05, 2.67, 0.0),
                ],
            ),
        ),
        SplinePoint::spline(-0.35, cubic_spline.clone()),
        SplinePoint::spline(-0.25, cubic_spline.clone()),
        SplinePoint::spline(
            -0.1,
            CubicSpline::new(
                SplineType::Ridges,
                vec![
                    SplinePoint::constant(-0.05, 2.67, 0.0),
                    SplinePoint::constant(0.05, 6.3, 0.0),
                ],
            ),
        ),
        SplinePoint::spline(0.03, cubic_spline.clone()),
    ];

    if higher_values {
        let cubic_spline1 = CubicSpline::new(
            SplineType::Ridges,
            vec![
                SplinePoint::constant(0.0, value, 0.0),
                SplinePoint::constant(0.1, 0.625, 0.0),
            ],
        );

        let cubic_spline2 = CubicSpline::new(
            SplineType::RidgesFolded,
            vec![
                SplinePoint::constant(-0.9, value, 0.0),
                SplinePoint::spline(-0.69, cubic_spline1),
            ],
        );

        points.extend(vec![
            SplinePoint::constant(0.35, value, 0.0),
            SplinePoint::spline(0.55, cubic_spline2.clone()),
            SplinePoint::constant(0.62, value, 0.0),
        ]);
    } else {
        let cubic_spline1 = CubicSpline::new(
            SplineType::RidgesFolded,
            vec![
                SplinePoint::spline(-0.7, cubic_spline.clone()),
                SplinePoint::constant(-0.15, 1.37, 0.0),
            ],
        );

        let cubic_spline2 = CubicSpline::new(
            SplineType::RidgesFolded,
            vec![
                SplinePoint::spline(0.45, cubic_spline),
                SplinePoint::constant(0.7, 1.56, 0.0),
            ],
        );

        points.extend(vec![
            SplinePoint::spline(0.05, cubic_spline2.clone()),
            SplinePoint::spline(0.4, cubic_spline2),
            SplinePoint::spline(0.45, cubic_spline1.clone()),
            SplinePoint::spline(0.55, cubic_spline1),
            SplinePoint::constant(0.58, value, 0.0),
        ]);
    }

    CubicSpline::new(SplineType::Erosion, points)
}

pub fn overworld_jaggedness() -> CubicSpline {
    CubicSpline::new(
        SplineType::Continents,
        vec![
            SplinePoint::constant(-0.11, 0.0, 0.0),
            SplinePoint::spline(0.03, build_erosion_jaggedness_spline(1.0, 0.5, 0.0, 0.0)),
            SplinePoint::spline(0.65, build_erosion_jaggedness_spline(1.0, 1.0, 1.0, 0.0)),
        ],
    )
}

fn build_erosion_jaggedness_spline(
    high_erosion_high_weirdness: f32,
    low_erosion_high_weirdness: f32,
    high_erosion_mid_weirdness: f32,
    low_erosion_mid_weirdness: f32,
) -> CubicSpline {
    let spline_high =
        build_ridge_jaggedness_spline(high_erosion_high_weirdness, high_erosion_mid_weirdness);

    let spline_low =
        build_ridge_jaggedness_spline(low_erosion_high_weirdness, low_erosion_mid_weirdness);

    CubicSpline::new(
        SplineType::RidgesFolded,
        vec![
            SplinePoint::spline(-1.0, spline_high),
            SplinePoint::spline(-0.78, spline_low.clone()),
            SplinePoint::spline(-0.5775, spline_low),
            SplinePoint::constant(-0.375, 0.0, 0.0),
        ],
    )
}

fn build_weirdness_jaggedness_spline(magnitude: f32) -> CubicSpline {
    CubicSpline::new(
        SplineType::Ridges,
        vec![
            SplinePoint::constant(-0.01, 0.63 * magnitude, 0.0),
            SplinePoint::constant(0.01, 0.3 * magnitude, 0.0),
        ],
    )
}

fn build_ridge_jaggedness_spline(
    high_weirdness_magnitude: f32,
    mid_weirdness_magnitude: f32,
) -> CubicSpline {
    fn peaks_and_valleys(weirdness: f32) -> f32 {
        -((weirdness.abs() - 0.666_666_7).abs() - 0.333_333_34) * 3.0
    }
    let f = peaks_and_valleys(0.4);
    let f1 = peaks_and_valleys(0.56666666);
    let f2 = (f + f1) / 2.0;

    let mid_point = if mid_weirdness_magnitude > 0.0 {
        SplinePoint::spline(
            f2,
            build_weirdness_jaggedness_spline(mid_weirdness_magnitude),
        )
    } else {
        SplinePoint::constant(f2, 0.0, 0.0)
    };

    let high_point = if high_weirdness_magnitude > 0.0 {
        SplinePoint::spline(
            1.0,
            build_weirdness_jaggedness_spline(high_weirdness_magnitude),
        )
    } else {
        SplinePoint::constant(1.0, 0.0, 0.0)
    };

    CubicSpline::new(
        SplineType::RidgesFolded,
        vec![SplinePoint::constant(f, 0.0, 0.0), mid_point, high_point],
    )
}

pub struct OverworldBiomeNoise {
    chunk_height: ChunkHeight,
    noise_size_vertical: usize,
    offset: CubicSpline,
    jaggedness: CubicSpline,
    factor: CubicSpline,
    shift: NormalNoise<4>,
    temperature: NormalNoise<6>,
    vegetation: NormalNoise<6>,
    continents: NormalNoise<9>,
    erosion: NormalNoise<5>,
    ridges: NormalNoise<6>,
    jagged: NormalNoise<16>,
    base_3d_noise_overworld: BlendedNoise,
    spaghetti_3d_rarity: NormalNoise<1>,
    spaghetti_3d_thickness: NormalNoise<1>,
    spaghetti_3d_1: NormalNoise<1>,
    spaghetti_3d_2: NormalNoise<1>,
    spaghetti_roughness: NormalNoise<1>,
    spaghetti_roughness_modulator: NormalNoise<1>,
    cave_entrance: NormalNoise<3>,
    spaghetti_2d_modulator: NormalNoise<1>,
    spaghetti_2d: NormalNoise<1>,
    spaghetti_2d_elevation: NormalNoise<1>,
    spaghetti_2d_thickness: NormalNoise<1>,
    pillar: NormalNoise<2>,
    pillar_rareness: NormalNoise<1>,
    pillar_thickness: NormalNoise<1>,
    cave_layer: NormalNoise<1>,
    cave_cheese: NormalNoise<9>,
    noodle: NormalNoise<1>,
    noodle_thickness: NormalNoise<1>,
    noodle_ridge_a: NormalNoise<1>,
    noodle_ridge_b: NormalNoise<1>,
}
impl OverworldBiomeNoise {
    pub fn new(factory: Xoroshiro128PlusPlus) -> Self {
        Self {
            chunk_height: ChunkHeight {
                min_y: -64,
                height: 384,
            },
            noise_size_vertical: 2 << 2,
            factor: overworld_factor(),
            jaggedness: overworld_jaggedness(),
            offset: get_offset_spline(),
            shift: SHIFT.init(factory),
            temperature: TEMPERATURE.init(factory),
            vegetation: VEGETATION.init(factory),
            continents: CONTINENTALNESS.init(factory),
            erosion: EROSION.init(factory),
            ridges: RIDGE.init(factory),
            jagged: JAGGED.init(factory),
            spaghetti_3d_1: SPAGHETTI_3D_1.init(factory),
            spaghetti_3d_rarity: SPAGHETTI_3D_RARITY.init(factory),
            spaghetti_3d_thickness: SPAGHETTI_3D_THICKNESS.init(factory),
            spaghetti_3d_2: SPAGHETTI_3D_2.init(factory),
            spaghetti_roughness: SPAGHETTI_ROUGHNESS.init(factory),
            spaghetti_roughness_modulator: SPAGHETTI_ROUGHNESS_MODULATOR.init(factory),
            cave_entrance: CAVE_ENTRANCE.init(factory),
            spaghetti_2d_modulator: SPAGHETTI_2D_MODULATOR.init(factory),
            spaghetti_2d: SPAGHETTI_2D.init(factory),
            spaghetti_2d_elevation: SPAGHETTI_2D_ELEVATION.init(factory),
            spaghetti_2d_thickness: SPAGHETTI_2D_THICKNESS.init(factory),
            pillar: PILLAR.init(factory),
            pillar_rareness: PILLAR_RARENESS.init(factory),
            pillar_thickness: PILLAR_THICKNESS.init(factory),
            cave_layer: CAVE_LAYER.init(factory),
            cave_cheese: CAVE_CHEESE.init(factory),
            noodle: NOODLE.init(factory),
            noodle_thickness: NOODLE_THICKNESS.init(factory),
            noodle_ridge_a: NOODLE_RIDGE_A.init(factory),
            noodle_ridge_b: NOODLE_RIDGE_B.init(factory),
            base_3d_noise_overworld: BASE_3D_NOISE_OVERWORLD
                .init(&mut factory.with_hash("minecraft:terrain")),
        }
    }

    ///TODO: always returns with y = 0; so we can cache anything that depends on this as input
    fn transform(&self, pos: DVec3) -> DVec3 {
        let noise_pos = pos.with_y(0.0);
        let shift_x = self.shift.at(noise_pos);
        let shift_z = self.shift.at(noise_pos.zxy());
        pos * DVec3::new(0.25, 0.0, 0.25) + DVec3::new(shift_x, 0.0, shift_z)
    }
    pub fn direct_preliminary_surface(&self, pos: ColumnPos) -> i32 {
        let spline_params = self.make_spline_params(self.transform(pos.block(0).into()));
        let factor = self.factor(spline_params) * 4.;
        let base_density = factor * self.offset(spline_params) - 0.703125;
        let res = (0.390625 - base_density) / factor;

        // y >= 30 / 8 * 8
        let y = res.remap(1.5, -1.5, -64., 320.0) as i32 / 8 * 8;

        if y >= 240 + 8 {
            for y in (240 + 8..=y.min(256 - 8)).rev().step_by(8) {
                let density = base_density + factor * f64::from(y).remap(-64.0, 320.0, 1.5, -1.5);
                let final_density = slide(
                    f64::from(y),
                    density,
                    240.0,
                    256.0,
                    -0.078125,
                    -64.0,
                    -40.0,
                    0.1171875,
                );
                if final_density > 0.390625 {
                    return y;
                }
            }
            y - 8
        } else {
            y
        }
    }

    pub fn initial_density_without_jaggedness(&self, pos: BlockPos) -> f64 {
        let spline_params = self.make_spline_params(self.transform(pos.into()));
        let mut factor_depth = self.factor(spline_params) * self.depth(pos, spline_params);
        factor_depth *= if factor_depth > 0.0 { 4.0 } else { 1.0 };
        let density = (factor_depth - 0.703125).clamp(-64.0, 64.0);
        slide(
            pos.y.into(),
            density,
            240.0,
            256.0,
            -0.078125,
            -64.0,
            -40.0,
            0.1171875,
        )
    }

    fn factor(&self, spline_params: (f64, f64, f64, f64)) -> f64 {
        f64::from(self.factor.sample(
            spline_params.0 as f32,
            spline_params.1 as f32,
            spline_params.2 as f32,
            spline_params.3 as f32,
        ))
    }
    fn entrances(&self, pos: DVec3, spaghetti_roughness: f64) -> f64 {
        let rarity = self.spaghetti_3d_rarity.at(pos * DVec3::new(2.0, 1.0, 2.0));
        let rarity = if rarity < -0.5 {
            0.75
        } else if rarity < 0.0 {
            1.0
        } else if rarity < 0.5 {
            1.5
        } else {
            2.0
        };
        let spaghetti_3d_thickness = self
            .spaghetti_3d_thickness
            .at(pos)
            .remap(-1.0, 1.0, -0.065, -0.088);
        let spaghetti_3d_1 = self.spaghetti_3d_1.at(pos / rarity).abs() * rarity;
        let spaghetti_3d_2 = self.spaghetti_3d_2.at(pos / rarity).abs() * rarity;
        let spaghetti_3d =
            (spaghetti_3d_1.max(spaghetti_3d_2) + spaghetti_3d_thickness).clamp(-1.0, 1.0);
        let cave_entrance = self.cave_entrance.at(pos * DVec3::new(0.75, 0.5, 0.75));
        let tmp = cave_entrance + 0.37 + clamped_map(pos.y, -10.0, 30.0, 0.3, 0.0);
        tmp.min(spaghetti_roughness + spaghetti_3d)
    }

    fn spaghetti_2d(&self, pos: DVec3) -> f64 {
        let spaghetti_roughness_modulator = self
            .spaghetti_2d_modulator
            .at(pos * DVec3::new(2.0, 1.0, 2.0));
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
        let spaghetti_2d = self.spaghetti_2d.at(pos / rarity).abs() * rarity;
        //TODO: cache
        let spaghetti_2d_elevation = self.spaghetti_2d_elevation.at(pos.with_y(0.0)) * 8.0;
        let tmp = (spaghetti_2d_elevation + pos.y.remap(-64.0, 320.0, 8.0, -40.0)).abs();
        let spaghetti_2d_thickness_modulator = self
            .spaghetti_2d_thickness
            .at(pos * DVec3::new(2.0, 1.0, 2.0))
            .remap(-1.0, 1.0, -0.6, -1.3);
        let thickness = (tmp + spaghetti_2d_thickness_modulator).powi(3);
        let tmp2 = spaghetti_2d + 0.083 * spaghetti_2d_thickness_modulator;
        thickness.max(tmp2).clamp(-1.0, 1.0)
    }
    fn pillars(&self, pos: DVec3) -> f64 {
        let pillar = self.pillar.at(pos * DVec3::new(25.0, 0.3, 25.0));
        let rareness = self.pillar_rareness.at(pos).remap(-1.0, 1.0, 0.0, -2.0);
        let thickness = self.pillar_thickness.at(pos).remap(-1.0, 1.0, 0.0, 1.1);
        thickness.powi(3) * (pillar * 2.0 + rareness)
    }
    fn underground(
        &self,
        sloped_cheese: f64,
        pos: DVec3,
        entrances: f64,
        spaghetti_roughness: f64,
    ) -> f64 {
        let spaghetti_2d = self.spaghetti_2d(pos);
        let cave_layer = self.cave_layer.at(pos * DVec3::new(1.0, 8.0, 1.0));
        let tmp = cave_layer.powi(2) * 4.0;
        let cave_cheese = self
            .cave_cheese
            .at(pos * DVec3::new(1.0, 0.6666666666666666, 1.0));
        let tmp2 =
            (cave_cheese + 0.27).clamp(-1.0, 1.0) + (1.5 + sloped_cheese * -0.64).clamp(0.0, 0.5);
        let f4 = tmp2 + tmp;
        let f5 = f4.min(entrances).min(spaghetti_roughness + spaghetti_2d);
        let pillars = self.pillars(pos);
        if pillars <= 0.03 { f5 } else { f5.max(pillars) }
    }

    fn spaghetti_roughness(&self, pos: DVec3) -> f64 {
        let initial_spaghetti_roughness = self.spaghetti_roughness.at(pos);
        let spaghetti_roughness_modulator = self
            .spaghetti_roughness_modulator
            .at(pos)
            .remap(-1.0, 1.0, 0.0, -0.1);
        (initial_spaghetti_roughness.abs() - 0.4) * spaghetti_roughness_modulator
    }
    fn noodle(&self, pos: DVec3) -> f64 {
        if pos.y < -60.0 || self.noodle.at(pos) <= 0.0 {
            return 64.0;
        }
        let thickness = self.noodle_thickness.at(pos).remap(-1.0, 1.0, -0.05, -0.1);
        let ridge_pos = pos * 2.6666666666666665;
        let ridge_a = self.noodle_ridge_a.at(ridge_pos);
        let ridge_b = self.noodle_ridge_b.at(ridge_pos);
        let ridge = ridge_a.abs().max(ridge_b.abs()) * 1.5;
        thickness + ridge
    }
    pub fn pre_baked_final_density(&self, pos: BlockPos) -> f64 {
        let transformed_pos = self.transform(pos.into());
        let spline_params = self.make_spline_params(transformed_pos);
        let jaggedness = self.jaggedness(spline_params);
        let jagged = self
            .jagged
            .at(pos.as_dvec3() * DVec3::new(1500.0, 0.0, 1500.0));

        let final_jaggedness = jagged * if jagged > 0.0 { 1.0 } else { 0.5 } * jaggedness;
        let depth =
            self.factor(spline_params) * (self.depth(pos, spline_params) + final_jaggedness);
        let base_3d_noise_overworld = self
            .base_3d_noise_overworld
            .at(pos.as_dvec3() * DVec3::new(0.25, 0.125, 0.25) * 684.412);
        let sloped_cheese = depth * if depth > 0.0 { 4.0 } else { 1.0 } + base_3d_noise_overworld;

        let spaghetti_roughness = self.spaghetti_roughness(pos.into());
        let entrances = self.entrances(pos.into(), spaghetti_roughness);
        let f8 = if sloped_cheese < 1.5625 {
            sloped_cheese.min(5.0 * entrances)
        } else {
            self.underground(sloped_cheese, pos.into(), entrances, spaghetti_roughness)
        };

        slide(
            pos.y.into(),
            f8,
            240.0,
            256.0,
            -0.078125,
            -64.0,
            -40.0,
            0.1171875,
        )
    }

    pub fn post_process(&self, pos: BlockPos, interpolated: f64) -> f64 {
        let d = (interpolated * 0.64).clamp(-1.0, 1.0);
        (d / 2.0 - d * d * d / 24.0).min(self.noodle(pos.into()))
    }

    fn jaggedness(&self, spline_params: (f64, f64, f64, f64)) -> f64 {
        f64::from(self.jaggedness.sample(
            spline_params.0 as f32,
            spline_params.1 as f32,
            spline_params.2 as f32,
            spline_params.3 as f32,
        ))
    }

    fn make_spline_params(&self, transformed_pos: DVec3) -> (f64, f64, f64, f64) {
        let ridges = self.ridges.at(transformed_pos);
        let ridges_folded = (ridges.abs() - 0.6666666666666666).abs() * -3.0 + 1.0;
        let erosion = self.erosion.at(transformed_pos);
        let continents = self.continents.at(transformed_pos);
        (ridges, ridges_folded, erosion, continents)
    }

    /// the last param (ridges) is not needed for this spline
    fn offset(&self, spline_params: (f64, f64, f64, f64)) -> f64 {
        f64::from(self.offset.sample(
            spline_params.0 as f32,
            spline_params.1 as f32,
            spline_params.2 as f32,
            spline_params.3 as f32,
        )) - 0.50375
    }

    /// the last param (ridges) is not needed
    fn depth(&self, pos: BlockPos, spline_params: (f64, f64, f64, f64)) -> f64 {
        let offset = self.offset(spline_params);
        f64::from(pos.y).remap(-64.0, 320.0, 1.5, -1.5) + offset
    }

    pub fn preliminary_surface(&self, chunk: ChunkPos) -> i32 {
        let column = chunk.column_pos(0, 0);
        self.chunk_height
            .iter()
            .rev()
            .step_by(self.noise_size_vertical)
            .find(|y| self.initial_density_without_jaggedness(column.block(*y)) > 0.390625)
            .unwrap_or(self.chunk_height.min_y)
    }
    pub fn is_deep_dark_region(&self, pos: IVec3) -> bool {
        let transformed_pos = self.transform(pos.into());
        self.erosion.at(transformed_pos) < EROSION_DEEP_DARK_DRYNESS_THRESHOLD.into()
            && self.depth(pos, self.make_spline_params(transformed_pos))
                > DEPTH_DEEP_DARK_DRYNESS_THRESHOLD.into()
    }
}
fn slide(
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
pub fn lerp3(
    delta: DVec3,
    c000: f64,
    c100: f64,
    c010: f64,
    c110: f64,
    c001: f64,
    c101: f64,
    c011: f64,
    c111: f64,
) -> f64 {
    let c00 = c000.lerp(c100, delta.x);
    let c10 = c010.lerp(c110, delta.x);
    let c01 = c001.lerp(c101, delta.x);
    let c11 = c011.lerp(c111, delta.x);
    let c0 = c00.lerp(c10, delta.y);
    let c1 = c01.lerp(c11, delta.y);
    c0.lerp(c1, delta.z)
}

pub fn generate_interpolation_data(
    biome_noise: &OverworldBiomeNoise,
    pos: ChunkPos,
    chunk: &mut Chunk,
) {
    use std::mem::swap;

    let mut slice0 = [[0.0; 5]; 5];
    let mut slice1 = [[0.0; 5]; 5];

    // initial base layer
    for (x, slice1x) in slice1.iter_mut().enumerate() {
        for (z, slice1xz) in slice1x.iter_mut().enumerate() {
            *slice1xz =
                biome_noise.pre_baked_final_density(pos.block(x as u32 * 4, -64, z as u32 * 4));
        }
    }

    for y in 1..48 {
        swap(&mut slice0, &mut slice1);

        for z in 0..5 {
            slice1[0][z] =
                biome_noise.pre_baked_final_density(pos.block(0, y * 8 - 64, z as u32 * 4));
        }

        for x in 1..4 {
            for z in 1..4 {
                slice1[x][z] = biome_noise.pre_baked_final_density(pos.block(
                    x as u32 * 4,
                    y * 8 - 64,
                    z as u32 * 4,
                ));

                let p000 = slice0[x - 1][z - 1];
                let p100 = slice0[x][z - 1];
                let p010 = slice0[x - 1][z];
                let p110 = slice0[x][z];
                let p001 = slice1[x - 1][z - 1];
                let p101 = slice1[x][z - 1];
                let p011 = slice1[x - 1][z];
                let p111 = slice1[x][z];

                for cy in 0..8 {
                    let fy = f64::from(cy) / 8.0;

                    // interpolate along Y for the bottom and top cubes
                    let bottom_y00 = p000.lerp(p010, fy);
                    let bottom_y10 = p100.lerp(p110, fy);
                    let top_y00 = p001.lerp(p011, fy);
                    let top_y10 = p101.lerp(p111, fy);

                    for cx in 0..4 {
                        let fx = f64::from(cx) / 4.0;

                        // interpolate along X for bottom/top surfaces
                        let bottom_xy = bottom_y00.lerp(bottom_y10, fx);
                        let top_xy = top_y00.lerp(top_y10, fx);

                        for cz in 0..4 {
                            let fz = f64::from(cz) / 4.0;
                            let res = bottom_xy.lerp(top_xy, fz);

                            let pos = BlockPos::new(
                                cx + (x as i32 - 1) * 4 + pos.pos.x,
                                cy + (y - 1) * 8 - 64,
                                cz + (z as i32 - 1) * 4 + pos.pos.y,
                            );
                            let res = biome_noise.post_process(pos, res);

                            chunk
                                .set_block(
                                    pos,
                                    if res > 0.0 {
                                        block!("stone")
                                    } else {
                                        block!("air")
                                    },
                                )
                                .unwrap();
                        }
                    }
                }
            }
        }
    }
}

impl BiomeNoise for OverworldBiomeNoise {
    fn at_inner(&self, pos: BlockPos) -> [f64; 6] {
        let transformed = self.transform(pos.into());
        let (ridges, ridges_folded, erosion, continents) = self.make_spline_params(transformed);
        [
            self.temperature.at(transformed),
            self.vegetation.at(transformed),
            continents,
            erosion,
            self.depth(pos, (ridges, ridges_folded, erosion, continents)),
            ridges,
        ]
    }
}

#[test]
fn test_offset() {
    let offset = get_offset_spline();
    // TODO:
    dbg!(offset.compute_min_max());
    dbg!(overworld_factor().compute_min_max());
    dbg!(overworld_jaggedness().compute_min_max());
    assert_eq!(offset.sample(0.0, 0.0, 0.0, 0.0), 0.007458158);
    assert_eq!(
        offset.sample(0.007458158, 0.007458158, 0.007458158, 0.007458158),
        0.008096008
    );
}
