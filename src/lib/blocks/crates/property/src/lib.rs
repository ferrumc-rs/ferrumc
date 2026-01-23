use std::str::FromStr;

mod simple;
mod double_block_half;
mod note_block_instrument;

pub use simple::*;
pub use double_block_half::DoubleBlockHalf;
pub use note_block_instrument::NoteBlockInstrument;

#[cfg(feature = "block-struct-generation")]
pub const TYPES: &[(&str, PropertyDescriptor)] = &[
    ("i32", PropertyDescriptor {
        matches_values: |str| str.parse::<i32>().is_ok(),
        ident_for: |str| {
            let val = str.parse::<i32>().expect("failed to parse i32");
            quote::quote! { #val }
        },
    }),
    ("bool", PropertyDescriptor {
        matches_values: |str| matches!(str, "true" | "false"),
        ident_for: |str| {
            let val = str.parse::<bool>().expect("failed to parse bool");
            quote::quote! { #val }
        }
    }),
    ("AttachFace", AttachFace::DESCRIPTOR),
    ("Axis", Axis::DESCRIPTOR),
    ("BambooLeaves", BambooLeaves::DESCRIPTOR),
    ("BedPart", BedPart::DESCRIPTOR),
    ("BellAttachType", BellAttachType::DESCRIPTOR),
    ("ChestType", ChestType::DESCRIPTOR),
    ("ComparatorMode", ComparatorMode::DESCRIPTOR),
    ("CreakingHeartState", CreakingHeartState::DESCRIPTOR),
    ("Direction", Direction::DESCRIPTOR),
    ("DoorHingeSide", DoorHingeSide::DESCRIPTOR),
    ("DoubleBlockHalf", DoubleBlockHalf::DESCRIPTOR),
    ("DripstoneThickness", DripstoneThickness::DESCRIPTOR),
    ("FrontAndTop", FrontAndTop::DESCRIPTOR),
    ("Half", Half::DESCRIPTOR),
    ("NoteBlockInstrument", NoteBlockInstrument::DESCRIPTOR),
    ("PistonType", PistonType::DESCRIPTOR),
    ("CopperGolemPose", CopperGolemPose::DESCRIPTOR),
    ("RailShape", RailShape::DESCRIPTOR),
    ("RedstoneSide", RedstoneSide::DESCRIPTOR),
    ("SculkSensorPhase", SculkSensorPhase::DESCRIPTOR),
    ("SideChainPart", SideChainPart::DESCRIPTOR),
    ("SlabType", SlabType::DESCRIPTOR),
    ("StairsShape", StairsShape::DESCRIPTOR),
    ("StructureMode", StructureMode::DESCRIPTOR),
    ("TestBlockMode", TestBlockMode::DESCRIPTOR),
    ("Tilt", Tilt::DESCRIPTOR),
    ("TrialSpawnerState", TrialSpawnerState::DESCRIPTOR),
    ("VaultState", VaultState::DESCRIPTOR),
    ("WallSide", WallSide::DESCRIPTOR),
];

#[cfg(feature = "block-struct-generation")]
pub struct PropertyDescriptor {
    pub matches_values: fn(&str) -> bool,
    pub ident_for: fn(&str) -> proc_macro2::TokenStream,
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