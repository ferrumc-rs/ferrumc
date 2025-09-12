use bevy_math::FloatExt;

use crate::overworld::spline::{CubicSpline, SplinePoint, SplineType};

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
    let f1 = -1.0;
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
pub(super) fn get_depth_spline() -> CubicSpline {
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
