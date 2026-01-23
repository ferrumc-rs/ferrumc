use std::str::FromStr;

mod simple;
mod double_block_half;
mod note_block_instrument;

pub use simple::*;
pub use double_block_half::DoubleBlockHalf;
pub use note_block_instrument::NoteBlockInstrument;

#[cfg(feature = "block-struct-generation")]
lazy_static::lazy_static! {
    pub static ref TYPES: std::collections::HashMap<&'static str, PropertyDescriptor> = {
        let mut map = std::collections::HashMap::new();

        map.insert("i32", PropertyDescriptor::I32);
        map.insert("bool", PropertyDescriptor::BOOL);
        map.insert("AttachFace", AttachFace::DESCRIPTOR);
        map.insert("Axis", Axis::DESCRIPTOR);
        map.insert("BambooLeaves", BambooLeaves::DESCRIPTOR);
        map.insert("BedPart", BedPart::DESCRIPTOR);
        map.insert("BellAttachType", BellAttachType::DESCRIPTOR);
        map.insert("ChestType", ChestType::DESCRIPTOR);
        map.insert("CreakingHeartState", CreakingHeartState::DESCRIPTOR);
        map.insert("ComparatorMode", ComparatorMode::DESCRIPTOR);
        map.insert("Direction", Direction::DESCRIPTOR);
        map.insert("DoorHingeSide", DoorHingeSide::DESCRIPTOR);
        map.insert("DoubleBlockHalf", DoubleBlockHalf::DESCRIPTOR);
        map.insert("DripstoneThickness", DripstoneThickness::DESCRIPTOR);
        map.insert("FrontAndTop", FrontAndTop::DESCRIPTOR);
        map.insert("Half", Half::DESCRIPTOR);
        map.insert("NoteBlockInstrument", NoteBlockInstrument::DESCRIPTOR);
        map.insert("PistonType", PistonType::DESCRIPTOR);
        map.insert("CopperGolemPose", CopperGolemPose::DESCRIPTOR);
        map.insert("RailShape", RailShape::DESCRIPTOR);
        map.insert("RedstoneSide", RedstoneSide::DESCRIPTOR);
        map.insert("SculkSensorPhase", SculkSensorPhase::DESCRIPTOR);
        map.insert("SideChainPart", SideChainPart::DESCRIPTOR);
        map.insert("SlabType", SlabType::DESCRIPTOR);
        map.insert("StairsShape", StairsShape::DESCRIPTOR);
        map.insert("StructureMode", StructureMode::DESCRIPTOR);
        map.insert("TestBlockMode", TestBlockMode::DESCRIPTOR);
        map.insert("Tilt", Tilt::DESCRIPTOR);
        map.insert("TrialSpawnerState", TrialSpawnerState::DESCRIPTOR);
        map.insert("VaultState", VaultState::DESCRIPTOR);
        map.insert("WallSide", WallSide::DESCRIPTOR);

        map
    };
}

#[cfg(feature = "block-struct-generation")]
pub struct PropertyDescriptor {
    pub matches_values: fn(&str) -> bool,
    pub ident_for: fn(&str) -> proc_macro2::TokenStream,
}

#[cfg(feature = "block-struct-generation")]
impl PropertyDescriptor {
    const I32: PropertyDescriptor = PropertyDescriptor {
        matches_values: |str| str.parse::<i32>().is_ok(),
        ident_for: |str| {
            let val = str.parse::<i32>().expect("failed to parse i32");
            quote::quote! { #val }
        },
    };

    const BOOL: PropertyDescriptor = PropertyDescriptor {
        matches_values: |str| matches!(str, "true" | "false"),
        ident_for: |str| {
            let val = str.parse::<bool>().expect("failed to parse bool");
            quote::quote! { #val }
        },
    };
}

/// Marker trait for types that can be used as block state property values
pub trait BlockStateProperty: FromStr + ToString {
    fn values(&self) -> &[&str] {
        &[]
    }
}

impl BlockStateProperty for i32 {}
impl BlockStateProperty for bool {}

/// Helper macro to implement enum property types
#[macro_export]
macro_rules! enum_property {
    ($name:ident, $($variant:ident => $variant_str:expr),* $(,)?) => {
        #[derive(Clone, Debug)]
        pub enum $name {
            $($variant),*
        }

        #[cfg(feature = "block-struct-generation")]
        impl $name {
            pub const DESCRIPTOR: $crate::PropertyDescriptor = $crate::PropertyDescriptor {
                matches_values: Self::matches_values,
                ident_for: Self::ident_for,
            };

            fn matches_values(str: &str) -> bool {
                matches!(str, $($variant_str)|*)
            }

            fn ident_for(str: &str) -> proc_macro2::TokenStream {
                match str {
                    $($variant_str => quote::quote!{ $name::$variant }),*,
                    str => panic!("{} is not a valid member of enum property {}", str, stringify!($name)),
                }
            }
        }

        impl std::str::FromStr for $name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($variant_str => Ok($name::$variant)),*,
                    s => Err(format!("Unknown property for {}: {}", stringify!($name), s)),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $($name::$variant => write!(f, $variant_str)),*
                }
            }
        }

        impl $crate::BlockStateProperty for $name {
            fn values(&self) -> &[&str] {
                &[$($variant_str),*]
            }
        }
    };
}