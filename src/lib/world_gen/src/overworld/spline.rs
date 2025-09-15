use bevy_math::FloatExt;

impl CubicSpline {
    pub fn new(spline_type: SplineType, points: Vec<SplinePoint>) -> Self {
        Self {
            spline_type,
            points,
        }
    }

    pub fn sample(&self, continents: f32, erosion: f32, ridges_folded: f32, ridges: f32) -> f32 {
        let x = match self.spline_type {
            SplineType::RidgesFolded => ridges_folded,
            SplineType::Ridges => ridges,
            SplineType::Continents => continents,
            SplineType::Erosion => erosion,
        };
        let n = self.points.len();
        assert!(n > 0);
        let i = match self.points.iter().rposition(|v| v.x <= x) {
            Some(idx) => idx,
            None => {
                // x is before the first point → extend linearly from 0
                return Self::linear_extend(
                    x,
                    &self.points[0],
                    continents,
                    erosion,
                    ridges_folded,
                    ridges,
                );
            }
        };

        if i == self.points.len() - 1 {
            // x is after the last point → extend from last
            return Self::linear_extend(
                x,
                &self.points[i],
                continents,
                erosion,
                ridges_folded,
                ridges,
            );
        }
        let point = &self.points[i];

        let point2 = &self.points[i + 1];
        let y0 = point.y.get(continents, erosion, ridges_folded, ridges);
        let y1 = point2.y.get(continents, erosion, ridges_folded, ridges);
        let dx = point2.x - point.x;
        let t = (x - point.x) / dx;

        let dy = y1 - y0;
        let h1 = point.slope * dx - dy;
        let h2 = -point2.slope * dx + dy;

        y0.lerp(y1, t) + t * (1.0 - t) * h1.lerp(h2, t)
    }

    fn linear_extend(
        x: f32,
        point: &SplinePoint,
        continents: f32,
        erosion: f32,
        ridges_folded: f32,
        ridges: f32,
    ) -> f32 {
        point.y.get(continents, erosion, ridges_folded, ridges) + point.slope * (x - point.x)
    }
}

#[derive(Debug, Clone)]
pub enum SplineValue {
    Const(f32),
    Spline(CubicSpline),
}

impl SplineValue {
    fn get(&self, continents: f32, erosion: f32, ridges_folded: f32, ridges: f32) -> f32 {
        match self {
            SplineValue::Const(res) => *res,
            SplineValue::Spline(cubic_spline) => {
                cubic_spline.sample(continents, erosion, ridges_folded, ridges)
            }
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
    assert_eq!(spline.sample(0.0, 0.0, 0.0, 0.0), -0.12)
}
