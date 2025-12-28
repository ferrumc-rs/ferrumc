#[derive(Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn to_i32(&self) -> i32 {
        ((self.r.clamp(0.0, 1.0) * 255.0) as i32) << 16
            | ((self.g.clamp(0.0, 1.0) * 255.0) as i32) << 8
            | ((self.b.clamp(0.0, 1.0) * 255.0) as i32)
    }
}
