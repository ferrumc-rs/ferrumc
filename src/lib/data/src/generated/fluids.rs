#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fluid {
    Empty,
    FlowingWater,
    Water,
    FlowingLava,
    Lava,
}
impl Fluid {
    #[doc = r" Try to parse a `Fluid` from a resource location string."]
    pub const fn try_from_name(name: &str) -> Option<Self> {
        let name = crate::helpers::strip_prefix_or_self(name, "minecraft:");
        match name {
            "empty" => Some(Self::Empty),
            "flowing_water" => Some(Self::FlowingWater),
            "water" => Some(Self::Water),
            "flowing_lava" => Some(Self::FlowingLava),
            "lava" => Some(Self::Lava),
            _ => None,
        }
    }
    #[doc = "Get name of the Fluid"]
    pub const fn to_name(&self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::FlowingWater => "flowing_water",
            Self::Water => "water",
            Self::FlowingLava => "flowing_lava",
            Self::Lava => "lava",
        }
    }
}
