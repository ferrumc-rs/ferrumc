use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Default)]
pub enum SlabType {
    Top,
    #[default]
    Bottom,
    Double,
}

impl Display for SlabType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SlabType::Top => write!(f, "top"),
            SlabType::Bottom => write!(f, "bottom"),
            SlabType::Double => write!(f, "double"),
        }
    }
}

impl FromStr for SlabType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "top" => Ok(SlabType::Top),
            "bottom" => Ok(SlabType::Bottom),
            "double" => Ok(SlabType::Double),
            _ => Err(format!("Unknown SlabType: {}", s)),
        }
    }
}