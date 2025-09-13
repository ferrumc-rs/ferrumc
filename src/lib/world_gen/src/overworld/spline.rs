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
            SplineType::Continents => erosion,
            SplineType::Erosion => continents,
        };
        let n = self.points.len();
        assert!(n > 0);

        // find i such that xs[i] <= x < xs[i+1]
        let Some((i, point)) = self.points.iter().enumerate().find(|(_, p)| p.x <= x) else {
            return Self::linear_extend(
                x,
                &self.points[n - 1],
                continents,
                erosion,
                ridges_folded,
                ridges,
            );
        };
        if i == n - 1 {
            return Self::linear_extend(x, point, continents, erosion, ridges_folded, ridges);
        }

        let point2 = &self.points[i + 1];
        let y0 = point.y.get(continents, erosion, ridges_folded, ridges);
        let y1 = point2.y.get(continents, erosion, ridges_folded, ridges);
        let t = (x - point.x) / (point2.x - point.x);

        let dx = point2.x - point.x;
        let h1 = point.slope * dx - (y1 - y0);
        let h2 = -point2.slope * dx + (y1 - y0);

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

#[derive(Clone)]
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

#[derive(Clone, Copy)]
pub enum SplineType {
    RidgesFolded,
    Ridges,
    Continents,
    Erosion,
}
/// One knot of the spline
#[derive(Clone)]
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

#[derive(Clone)]
pub struct CubicSpline {
    spline_type: SplineType,
    pub points: Vec<SplinePoint>, //TODO: const maybe
}
