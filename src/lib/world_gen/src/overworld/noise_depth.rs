use crate::biome_chunk::BiomeNoise;
use crate::overworld::aquifer::clamped_map;
use crate::perlin_noise::{
    BASE_3D_NOISE_OVERWORLD, BlendedNoise, CAVE_CHEESE, CAVE_ENTRANCE, CAVE_LAYER, CONTINENTALNESS,
    EROSION, JAGGED, NOODLE, NOODLE_RIDGE_A, NOODLE_RIDGE_B, NOODLE_THICKNESS, NormalNoise, PILLAR,
    PILLAR_RARENESS, PILLAR_THICKNESS, RIDGE, SHIFT, SPAGHETTI_2D, SPAGHETTI_2D_ELEVATION,
    SPAGHETTI_2D_MODULATOR, SPAGHETTI_2D_THICKNESS, SPAGHETTI_3D_1, SPAGHETTI_3D_2,
    SPAGHETTI_3D_RARITY, SPAGHETTI_3D_THICKNESS, SPAGHETTI_ROUGHNESS,
    SPAGHETTI_ROUGHNESS_MODULATOR, TEMPERATURE, VEGETATION,
};
use crate::pos::BlockPos;
use crate::random::RngFactory;
use bevy_math::{DVec3, FloatExt, Vec3Swizzles};

use crate::{
    overworld::spline::{CubicSpline, SplinePoint, SplineType},
    random::Xoroshiro128PlusPlusFactory,
};

fn build_erosion_offset_spline(
    f: f32,
    f1: f32,
    f2: f32,
    magnitude: f32,
    f3: f32,
    f4: f32,
    extended: bool,
    use_max_slope: bool,
) -> CubicSpline {
    // Build the intermediate splines
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
    let cubic7 = {
        let pts = vec![
            SplinePoint::constant(-1.0, f, 0.0),
            SplinePoint::spline(-0.4, cubic5.clone(), 0.0),
            SplinePoint::constant(0.0, f2 + 0.07, 0.0),
        ];
        CubicSpline::new(SplineType::RidgesFolded, pts)
    };
    let cubic8 = ridge_spline(-0.02, f4, f4, f1, f2, 0.0);

    // Build the main spline
    let mut points = vec![
        SplinePoint::spline(-0.85, cubic, 0.0),
        SplinePoint::spline(-0.7, cubic1, 0.0),
        SplinePoint::spline(-0.4, cubic2, 0.0),
        SplinePoint::spline(-0.35, cubic3, 0.0),
        SplinePoint::spline(-0.1, cubic4, 0.0),
        SplinePoint::spline(0.2, cubic5, 0.0),
    ];

    if extended {
        points.extend(vec![
            SplinePoint::spline(0.4, cubic6.clone(), 0.0),
            SplinePoint::spline(0.45, cubic7.clone(), 0.0),
            SplinePoint::spline(0.55, cubic7.clone(), 0.0),
            SplinePoint::spline(0.58, cubic6.clone(), 0.0),
        ]);
    }

    points.push(SplinePoint::spline(0.7, cubic8, 0.0));

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
        let f10 = calculate_slope(f2, f9, -1.0, f8);

        points.push(SplinePoint::constant(-1.0, f2, f10));
        points.push(SplinePoint::constant(f8, f9, 0.0));
        points.push(SplinePoint::constant(f6, f7, 0.0));

        let f11 = mountain_continentalness(f5, magnitude, -0.7);
        let f12 = calculate_slope(f11, f4, f5, 1.0);

        points.push(SplinePoint::constant(f5 - 0.01, f11, 0.0));
        points.push(SplinePoint::constant(f5, f11, f12));
        points.push(SplinePoint::constant(1.0, f4, f12));
    } else {
        let f7 = calculate_slope(f2, f4, -1.0, 1.0);

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

// Helper functions
fn calculate_slope(y1: f32, y2: f32, x1: f32, x2: f32) -> f32 {
    (y2 - y1) / (x2 - x1)
}
pub(super) fn get_offset_spline() -> CubicSpline {
    let cubic = build_erosion_offset_spline(-0.15, 0.0, 0.0, 0.1, 0.0, -0.03, false, false);
    let cubic1 = build_erosion_offset_spline(-0.1, 0.03, 0.1, 0.1, 0.01, -0.03, false, false);
    let cubic2 = build_erosion_offset_spline(-0.1, 0.03, 0.1, 0.7, 0.01, -0.03, true, true);
    let cubic3 = build_erosion_offset_spline(-0.05, 0.03, 0.1, 1.0, 0.01, 0.01, true, true);

    CubicSpline::new(
        SplineType::Continents,
        vec![
            SplinePoint::constant(-1.1, 0.044, 0.0),
            SplinePoint::constant(-1.02, -0.2222, 0.0),
            SplinePoint::constant(-0.51, -0.2222, 0.0),
            SplinePoint::constant(-0.44, -0.12, 0.0),
            SplinePoint::constant(-0.18, -0.12, 0.0),
            SplinePoint::spline(-0.16, cubic.clone(), 0.0),
            SplinePoint::spline(-0.15, cubic.clone(), 0.0),
            SplinePoint::spline(-0.10, cubic1.clone(), 0.0),
            SplinePoint::spline(0.25, cubic2.clone(), 0.0),
            SplinePoint::spline(1.0, cubic3, 0.0),
        ],
    )
}

pub fn overworld_factor() -> CubicSpline {
    CubicSpline::new(
        SplineType::Continents,
        vec![
            SplinePoint::constant(-0.19, 3.95, 0.0),
            SplinePoint::spline(-0.15, get_erosion_factor(6.25, true), 0.0),
            SplinePoint::spline(-0.1, get_erosion_factor(5.47, true), 0.0),
            SplinePoint::spline(0.03, get_erosion_factor(5.08, true), 0.0),
            SplinePoint::spline(0.06, get_erosion_factor(4.69, false), 0.0),
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
        SplinePoint::spline(-0.6, cubic_spline.clone(), 0.0),
        SplinePoint::spline(
            -0.5,
            CubicSpline::new(
                SplineType::Ridges,
                vec![
                    SplinePoint::constant(-0.05, 6.3, 0.0),
                    SplinePoint::constant(0.05, 2.67, 0.0),
                ],
            ),
            0.0,
        ),
        SplinePoint::spline(-0.35, cubic_spline.clone(), 0.0),
        SplinePoint::spline(-0.25, cubic_spline.clone(), 0.0),
        SplinePoint::spline(
            -0.1,
            CubicSpline::new(
                SplineType::Ridges,
                vec![
                    SplinePoint::constant(-0.05, 2.67, 0.0),
                    SplinePoint::constant(0.05, 6.3, 0.0),
                ],
            ),
            0.0,
        ),
        SplinePoint::spline(0.03, cubic_spline.clone(), 0.0),
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
                SplinePoint::spline(-0.69, cubic_spline1, 0.0),
            ],
        );

        points.extend(vec![
            SplinePoint::constant(0.35, value, 0.0),
            SplinePoint::spline(0.45, cubic_spline2.clone(), 0.0),
            SplinePoint::spline(0.55, cubic_spline2.clone(), 0.0),
            SplinePoint::constant(0.62, value, 0.0),
        ]);
    } else {
        let cubic_spline1 = CubicSpline::new(
            SplineType::RidgesFolded,
            vec![
                SplinePoint::spline(-0.7, cubic_spline.clone(), 0.0),
                SplinePoint::constant(-0.15, 1.37, 0.0),
            ],
        );

        let cubic_spline2 = CubicSpline::new(
            SplineType::RidgesFolded,
            vec![
                SplinePoint::spline(0.45, cubic_spline.clone(), 0.0),
                SplinePoint::constant(0.7, 1.56, 0.0),
            ],
        );

        points.extend(vec![
            SplinePoint::spline(0.05, cubic_spline2.clone(), 0.0),
            SplinePoint::spline(0.4, cubic_spline2.clone(), 0.0),
            SplinePoint::spline(0.45, cubic_spline1.clone(), 0.0),
            SplinePoint::spline(0.55, cubic_spline1.clone(), 0.0),
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
            SplinePoint::spline(
                0.03,
                build_erosion_jaggedness_spline(1.0, 0.5, 0.0, 0.0),
                0.0,
            ),
            SplinePoint::spline(
                0.65,
                build_erosion_jaggedness_spline(1.0, 1.0, 1.0, 0.0),
                0.0,
            ),
        ],
    )
}

fn build_erosion_jaggedness_spline(
    high_erosion_high_weirdness: f32,
    low_erosion_high_weirdness: f32,
    high_erosion_mid_weirdness: f32,
    low_erosion_mid_weirdness: f32,
) -> CubicSpline {
    let f = -0.5775;

    let spline_high =
        build_ridge_jaggedness_spline(high_erosion_high_weirdness, high_erosion_mid_weirdness);

    let spline_low =
        build_ridge_jaggedness_spline(low_erosion_high_weirdness, low_erosion_mid_weirdness);

    CubicSpline::new(
        SplineType::RidgesFolded,
        vec![
            SplinePoint::spline(-1.0, spline_high, 0.0),
            SplinePoint::spline(-0.78, spline_low.clone(), 0.0),
            SplinePoint::spline(f, spline_low, 0.0),
            SplinePoint::constant(-0.375, 0.0, 0.0),
        ],
    )
}

fn build_weirdness_jaggedness_spline(magnitude: f32) -> CubicSpline {
    let f = 0.63 * magnitude;
    let f1 = 0.3 * magnitude;

    CubicSpline::new(
        SplineType::Ridges,
        vec![
            SplinePoint::constant(-0.01, f, 0.0),
            SplinePoint::constant(0.01, f1, 0.0),
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
            0.0,
        )
    } else {
        SplinePoint::constant(f2, 0.0, 0.0)
    };

    let high_point = if high_weirdness_magnitude > 0.0 {
        SplinePoint::spline(
            1.0,
            build_weirdness_jaggedness_spline(high_weirdness_magnitude),
            0.0,
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
    pub(super) fn new(random: Xoroshiro128PlusPlusFactory) -> Self {
        Self {
            factor: overworld_factor(),
            jaggedness: overworld_jaggedness(),
            offset: get_offset_spline(),
            shift: SHIFT.init(random),
            temperature: TEMPERATURE.init(random),
            vegetation: VEGETATION.init(random),
            continents: CONTINENTALNESS.init(random),
            erosion: EROSION.init(random),
            ridges: RIDGE.init(random),
            jagged: JAGGED.init(random),
            spaghetti_3d_1: SPAGHETTI_3D_1.init(random),
            spaghetti_3d_rarity: SPAGHETTI_3D_RARITY.init(random),
            spaghetti_3d_thickness: SPAGHETTI_3D_THICKNESS.init(random),
            spaghetti_3d_2: SPAGHETTI_3D_2.init(random),
            spaghetti_roughness: SPAGHETTI_ROUGHNESS.init(random),
            spaghetti_roughness_modulator: SPAGHETTI_ROUGHNESS_MODULATOR.init(random),
            cave_entrance: CAVE_ENTRANCE.init(random),
            spaghetti_2d_modulator: SPAGHETTI_2D_MODULATOR.init(random),
            spaghetti_2d: SPAGHETTI_2D.init(random),
            spaghetti_2d_elevation: SPAGHETTI_2D_ELEVATION.init(random),
            spaghetti_2d_thickness: SPAGHETTI_2D_THICKNESS.init(random),
            pillar: PILLAR.init(random),
            pillar_rareness: PILLAR_RARENESS.init(random),
            pillar_thickness: PILLAR_THICKNESS.init(random),
            cave_layer: CAVE_LAYER.init(random),
            cave_cheese: CAVE_CHEESE.init(random),
            noodle: NOODLE.init(random),
            noodle_thickness: NOODLE_THICKNESS.init(random),
            noodle_ridge_a: NOODLE_RIDGE_A.init(random),
            noodle_ridge_b: NOODLE_RIDGE_B.init(random),
            base_3d_noise_overworld: BASE_3D_NOISE_OVERWORLD
                .init(&mut random.with_hash("minecraft:terrain")),
        }
    }

    pub fn transform(&self, pos: BlockPos) -> DVec3 {
        let shift_x = self.shift.at(pos.with_y(0).into());
        let shift_z = self.shift.at(pos.with_y(0).zxy().into());
        pos.as_dvec3() * DVec3::new(0.25, 1.0, 0.25) + DVec3::new(shift_x, 0.0, shift_z)
    }
    pub fn initial_density_without_jaggedness(&self, pos: BlockPos) -> f64 {
        let transformed_pos = self.transform(pos);
        let spline_params = self.make_spline_params(transformed_pos);
        let factor = self.factor(spline_params);
        let mut factor_depth = factor * self.depth(pos, spline_params);
        factor_depth *= if factor_depth > 0.0 { 4.0 } else { 1.0 };
        let density = (factor_depth - 0.703125).clamp(-64.0, 64.0);
        slide(
            pos.y, density, -64, 384, 80, 64, -0.078125, 0, 24, 0.1171875,
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
        let spaghetti_2d_elevation =
            self.spaghetti_2d_elevation
                .at(pos)
                .remap(-1.0, 1.0, -64f64.div_euclid(8.0), 8.0);
        let tmp = (spaghetti_2d_elevation + clamped_map(pos.y, -64.0, 320.0, 8.0, -40.0)).abs();
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
        let pillar_rareness = self.pillar_rareness.at(pos).remap(-1.0, 1.0, 0.0, -2.0);
        let pillar_thickness = self.pillar_thickness.at(pos).remap(-1.0, 1.0, 0.0, 1.1);
        pillar_thickness.powi(3) * (pillar * 2.0 + pillar_rareness)
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
        if pos.y < -60.0 {
            return 64.0;
        }
        let noodle = self.noodle.at(pos);
        let noodle_thickness = self.noodle_thickness.at(pos).remap(-1.0, 1.0, -0.05, -0.1);
        let noodle_ridge_a = self.noodle_ridge_a.at(pos * 2.6666666666666665);
        let noodle_ridge_b = self.noodle_ridge_b.at(pos * 2.6666666666666665);
        let noodle_ridge = noodle_ridge_a.abs().max(noodle_ridge_b.abs()) * 1.5;
        if noodle <= 0.0 {
            64.0
        } else {
            noodle_thickness + noodle_ridge
        }
    }
    pub fn final_density(&self, pos: BlockPos) -> f64 {
        let transformed_pos = self.transform(pos);
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
        let f7 = sloped_cheese.min(5.0 * entrances);
        let f8 = if sloped_cheese < 1.5625 {
            f7
        } else {
            self.underground(sloped_cheese, pos.into(), entrances, spaghetti_roughness)
        };

        let tmp = slide(pos.y, f8, -64, 384, 80, 64, -0.078125, 0, 24, 0.1171875);

        let blended = tmp; //TODO: blender.blend_density(pos, tmp); //interpolated

        let d = (blended * 0.64).clamp(-1.0, 1.0);

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

    pub fn make_spline_params(&self, transformed_pos: DVec3) -> (f64, f64, f64, f64) {
        let ridges = self.ridges.at(transformed_pos);
        let ridges_folded = ((ridges.abs() - 0.6666666666666666).abs() - 0.3333333333333333) * -3.0;
        let erosion = self.erosion.at(transformed_pos);
        let continents = self.continents.at(transformed_pos);
        (ridges, ridges_folded, erosion, continents)
    }

    fn offset(&self, spline_params: (f64, f64, f64, f64)) -> f64 {
        BLEND_OFFSET.lerp(
            -0.50375
                + f64::from(self.offset.sample(
                    spline_params.0 as f32,
                    spline_params.1 as f32,
                    spline_params.2 as f32,
                    spline_params.3 as f32,
                )),
            BLEND_ALPHA,
        )
    }
    fn temperature(&self, pos: BlockPos) -> f64 {
        self.temperature.at(self.transform(pos))
    }
    fn vegetation(&self, pos: BlockPos) -> f64 {
        self.vegetation.at(self.transform(pos))
    }

    fn continents(&self, pos: BlockPos) -> f64 {
        self.continents.at(self.transform(pos))
    }

    pub fn erosion(&self, pos: BlockPos) -> f64 {
        self.erosion.at(self.transform(pos))
    }

    pub fn depth(&self, pos: BlockPos, spline_params: (f64, f64, f64, f64)) -> f64 {
        let offset = self.offset(spline_params);
        f64::from(pos.y).remap(-64.0, 320.0, 1.5, -1.5) + offset
    }

    fn ridges(&self, pos: BlockPos) -> f64 {
        self.ridges.at(self.transform(pos))
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
    let s = clamped_map(
        f64::from(y),
        f64::from(min_y + height - top_start_offset),
        f64::from(min_y + height - top_end_offset),
        1.0,
        0.0,
    );
    let t = clamped_map(
        f64::from(y),
        f64::from(min_y + bottom_start_offset),
        f64::from(min_y + bottom_end_offset),
        0.0,
        1.0,
    );
    bottom_delta.lerp(top_delta.lerp(density, s), t)
}
const BLEND_ALPHA: f64 = 1.0;
const BLEND_OFFSET: f64 = 0.0;
impl BiomeNoise for OverworldBiomeNoise {
    fn at_inner(&self, pos: BlockPos) -> [f64; 6] {
        let transformed = self.transform(pos);
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
