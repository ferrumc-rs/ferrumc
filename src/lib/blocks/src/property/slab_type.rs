use std::fmt::Display;
use std::str::FromStr;
use crate::property::BlockStateProperty;

#[derive(Default)]
pub enum SlabType {
    Top,
    #[default]
    Bottom,
    Double,
}

impl FromStr for SlabType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "top" => Ok(SlabType::Top),
            "bottom" => Ok(SlabType::Bottom),
            "double" => Ok(SlabType::Double),
            _ => Err(format!("invalid slab type: {}", s)),
        }
    }
}

impl Display for SlabType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SlabType::Top => write!(f, "top"),
            SlabType::Bottom => write!(f, "bottom"),
            SlabType::Double => write!(f, "double"),
        }
    }
}

impl BlockStateProperty for SlabType {}