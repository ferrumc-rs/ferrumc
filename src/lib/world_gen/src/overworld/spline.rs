use bevy_math::FloatExt;

#[derive(Clone, Copy)]
pub struct SplineCoord {
    pub continents: f64,
    pub erosion: f64,
    pub ridges_folded: f64,
    pub ridges: f64,
}

impl SplineCoord {
    pub fn new(continents: f64, erosion: f64, ridges_folded: f64, ridges: f64) -> Self {
        Self {
            continents,
            erosion,
            ridges_folded,
            ridges,
        }
    }
}

impl CubicSpline {
    pub fn new(spline_type: SplineType, points: Vec<SplinePoint>) -> Self {
        Self {
            spline_type,
            points,
        }
    }

    pub fn sample(&self, coord: SplineCoord) -> f32 {
        let x = match self.spline_type {
            SplineType::RidgesFolded => coord.ridges_folded,
            SplineType::Ridges => coord.ridges,
            SplineType::Continents => coord.continents,
            SplineType::Erosion => coord.erosion,
        } as f32;
        let n = self.points.len();
        assert!(n > 0);
        let i = match self.points.iter().rposition(|v| v.x <= x) {
            Some(idx) => idx,
            None => {
                // x is before the first point → extend linearly from 0
                return Self::linear_extend(x, &self.points[0], self.points[0].y.get(coord));
            }
        };

        if i == self.points.len() - 1 {
            // x is after the last point → extend from last
            return Self::linear_extend(x, &self.points[i], self.points[i].y.get(coord));
        }
        let point = &self.points[i];

        let point2 = &self.points[i + 1];
        let y0 = point.y.get(coord);
        let y1 = point2.y.get(coord);
        let dx = point2.x - point.x;
        let t = (x - point.x) / dx;

        let dy = y1 - y0;
        let h1 = point.slope * dx - dy;
        let h2 = -point2.slope * dx + dy;

        y0.lerp(y1, t) + t * (1.0 - t) * h1.lerp(h2, t)
    }

    fn linear_extend(x: f32, point: &SplinePoint, y: f32) -> f32 {
        y + point.slope * (x - point.x)
    }

    pub fn compute_min_max(&self) -> (f32, f32) {
        let coordinate_min = self.points.first().map(|p| p.x).unwrap_or(0.0);
        let coordinate_max = self.points.last().map(|p| p.x).unwrap_or(0.0);
        let points = &self.points;
        let n = points.len() - 1;

        let mut min_val = f32::INFINITY;
        let mut max_val = f32::NEG_INFINITY;

        // --- Handle extension below first knot
        if coordinate_min < points[0].x {
            let p = &points[0];
            let (y_min, y_max) = match &p.y {
                SplineValue::Const(v) => {
                    let linear_extend = Self::linear_extend(coordinate_min, p, *v);
                    (linear_extend, linear_extend)
                }
                SplineValue::Spline(s) => s.compute_min_max(),
            };
            min_val = min_val.min(y_min.min(y_max));
            max_val = max_val.max(y_min.max(y_max));
        }

        // --- Handle extension above last knot
        if coordinate_max > points[n].x {
            let p = &points[n];
            let (y_min, y_max) = match &p.y {
                SplineValue::Const(v) => {
                    let linear_extend = Self::linear_extend(coordinate_min, p, *v);
                    (linear_extend, linear_extend)
                }
                SplineValue::Spline(s) => s.compute_min_max(),
            };
            min_val = min_val.min(y_min.min(y_max));
            max_val = max_val.max(y_min.max(y_max));
        }

        // --- Check all sub-splines
        for p in points {
            match &p.y {
                SplineValue::Const(v) => {
                    min_val = min_val.min(*v);
                    max_val = max_val.max(*v);
                }
                SplineValue::Spline(s) => {
                    let (min, max) = s.compute_min_max();
                    min_val = min_val.min(min);
                    max_val = max_val.max(max);
                }
            }
        }

        // --- Check each interval between points
        for i in 0..n {
            let p0 = &points[i];
            let p1 = &points[i + 1];
            let dx = p1.x - p0.x;
            let d0 = p0.slope;
            let d1 = p1.slope;

            // Nested spline handling
            let (y0_min, y0_max) = match &p0.y {
                SplineValue::Const(v) => (*v, *v),
                SplineValue::Spline(s) => s.compute_min_max(),
            };
            let (y1_min, y1_max) = match &p1.y {
                SplineValue::Const(v) => (*v, *v),
                SplineValue::Spline(s) => s.compute_min_max(),
            };

            if d0 != 0.0 || d1 != 0.0 {
                let d0_scaled = d0 * dx;
                let d1_scaled = d1 * dx;
                let min_y = y0_min.min(y1_min);
                let max_y = y0_max.max(y1_max);

                let f16 = d0_scaled - y1_max + y0_min;
                let f17 = d0_scaled - y1_min + y0_max;
                let f18 = -d1_scaled + y1_min - y0_max;
                let f19 = -d1_scaled + y1_max - y0_min;

                let inner_min = f16.min(f18);
                let inner_max = f17.max(f19);

                min_val = min_val.min(min_y + 0.25 * inner_min);
                max_val = max_val.max(max_y + 0.25 * inner_max);
            }
        }

        (min_val, max_val)
    }
}

#[derive(Debug, Clone)]
pub enum SplineValue {
    Const(f32),
    Spline(CubicSpline),
}

impl SplineValue {
    fn get(&self, coord: SplineCoord) -> f32 {
        match self {
            SplineValue::Const(res) => *res,
            SplineValue::Spline(cubic_spline) => cubic_spline.sample(coord),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SplineType {
    RidgesFolded,
    Ridges,
    Continents,
    Erosion,
}
/// One knot of the spline
#[derive(Debug, Clone)]
pub struct SplinePoint {
    pub x: f32,
    pub y: SplineValue,
    pub slope: f32,
}

impl SplinePoint {
    pub fn constant(x: f32, y: f32, slope: f32) -> Self {
        Self {
            x,
            y: SplineValue::Const(y),
            slope,
        }
    }
    pub fn spline(x: f32, y: CubicSpline) -> Self {
        Self {
            x,
            y: SplineValue::Spline(y),
            slope: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CubicSpline {
    spline_type: SplineType,
    pub points: Vec<SplinePoint>, //TODO: const maybe
}

#[test]
fn test_spline() {
    let cubic3 = CubicSpline::new(
        SplineType::Continents,
        vec![
            SplinePoint::constant(-1.1, 0.044, 0.0),
            SplinePoint::constant(-1.02, -0.2222, 0.0),
            SplinePoint::constant(-0.51, -0.2222, 0.0),
            SplinePoint::constant(-0.44, -0.12, 0.0),
            SplinePoint::constant(-0.18, -0.12, 0.0),
        ],
    );
    let spline = CubicSpline::new(
        SplineType::Continents,
        vec![
            SplinePoint::constant(-1.1, 0.044, 0.0),
            SplinePoint::constant(-1.02, -0.2222, 0.0),
            SplinePoint::constant(-0.51, -0.2222, 0.0),
            SplinePoint::constant(-0.44, -0.12, 0.0),
            SplinePoint::constant(-0.18, -0.12, 0.0),
            SplinePoint::spline(1.0, cubic3),
        ],
    );
    assert_eq!(spline.sample(SplineCoord::new(0.0, 0.0, 0.0, 0.0)), -0.12)
}
