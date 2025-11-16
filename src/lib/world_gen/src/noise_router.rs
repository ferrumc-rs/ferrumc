// Topo-sort this first!!!
pub struct MultiNoiseRouter {
    pub full_component_stack: Box<[BaseNoiseFunction]>,
    pub temperature: usize,
    pub vegetation: usize,
    pub continents: usize,
    pub erosion: usize,
    pub depth: usize,
    pub ridges: usize,
}

impl MultiNoiseRouter {
    fn process(&self) {
        let mut values: Vec<f64> = Vec::with_capacity(self.full_component_stack.len());
        for node in &self.full_component_stack {
            values.push(node.process(&values));
        }
    }
}

pub enum BaseNoiseFunction {
    // One argument
    Abs { argument: usize },
    Square { argument: usize },
    Cube { argument: usize },
    HalfNegative { argument: usize },
    QuarterNegative { argument: usize },
    Squeeze { argument: usize },
    Invert { argument: usize },

    // Two arguments
    Add { arg1: usize, arg2: usize },
    Mul { arg1: usize, arg2: usize },
    Min { arg1: usize, arg2: usize },
    Max { arg1: usize, arg2: usize },

    // Others
    Constant { value: f64 },
    EndIslands,
    // ClampedYGradient { from_y: i32, to_y: i32 },
    Shift,
    ShiftA,
    ShiftB,

    Clamp { input: usize, min: f64, max: f64 },
    // Shouldn't be used???
    Beardifier,
}

impl BaseNoiseFunction {
    fn process(&self, stack: &[f64]) -> f64 {
        match *self {
            // One argument
            Self::Abs { argument } => stack[argument].abs(),
            Self::Square { argument } => stack[argument].powi(2),
            Self::Cube { argument } => stack[argument].powi(3),
            Self::HalfNegative { argument } => {
                let x = stack[argument];
                if x.is_sign_negative() { x / 2.0 } else { x }
            }
            Self::QuarterNegative { argument } => {
                let x = stack[argument];
                if x.is_sign_negative() { x / 4.0 } else { x }
            }
            Self::Squeeze { argument } => {
                let x = stack[argument].clamp(-1.0, 1.0);
                // I'm too bad at math/too lazy to actually make this readable
                x / 2.0 - x * x * x / 24.0
            }
            Self::Invert { argument } => 1.0 / stack[argument],

            // Two arguments
            Self::Add { arg1, arg2 } => stack[arg1] + stack[arg2],
            Self::Mul { arg1, arg2 } => stack[arg1] * stack[arg2],
            Self::Min { arg1, arg2 } => stack[arg1].min(stack[arg2]),
            Self::Max { arg1, arg2 } => stack[arg1].max(stack[arg2]),
            _ => todo!(),
        }
    }
}
