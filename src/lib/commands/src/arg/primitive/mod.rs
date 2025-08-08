//! Primitive command argument types.
// TODO:
// * Entity
// * Double (f64)
// * Score Holder
// * Time
// * Resource or Tag
// * Resource or Tag Key
// * Resource
// * Resource Key

use std::io::Write;

use enum_ordinalize::Ordinalize;
use ferrumc_macros::NetEncode;
use ferrumc_net_codec::{
    encode::{errors::NetEncodeError, NetEncode, NetEncodeOpts},
    net_types::var_int::VarInt,
};
use float::FloatArgumentFlags;
use int::IntArgumentFlags;
use long::LongArgumentFlags;
use string::StringArgumentType;
use tokio::io::AsyncWrite;

pub mod float;
pub mod int;
pub mod long;
pub mod string;

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveArgument {
    pub argument_type: PrimitiveArgumentType,
    pub flags: Option<PrimitiveArgumentFlags>,
}

impl PrimitiveArgument {
    pub fn word() -> PrimitiveArgument {
        PrimitiveArgument {
            argument_type: PrimitiveArgumentType::String,
            flags: Some(PrimitiveArgumentFlags::String(StringArgumentType::Word)),
        }
    }

    pub fn quotable() -> PrimitiveArgument {
        PrimitiveArgument {
            argument_type: PrimitiveArgumentType::String,
            flags: Some(PrimitiveArgumentFlags::String(StringArgumentType::Quotable)),
        }
    }

    pub fn greedy() -> PrimitiveArgument {
        PrimitiveArgument {
            argument_type: PrimitiveArgumentType::String,
            flags: Some(PrimitiveArgumentFlags::String(StringArgumentType::Greedy)),
        }
    }

    pub fn int(min: Option<i32>, max: Option<i32>) -> PrimitiveArgument {
        PrimitiveArgument {
            argument_type: PrimitiveArgumentType::Int,
            flags: Some(PrimitiveArgumentFlags::Int(IntArgumentFlags { min, max })),
        }
    }

    pub fn long(min: Option<i64>, max: Option<i64>) -> PrimitiveArgument {
        PrimitiveArgument {
            argument_type: PrimitiveArgumentType::Long,
            flags: Some(PrimitiveArgumentFlags::Long(LongArgumentFlags { min, max })),
        }
    }

    pub fn float(min: Option<f32>, max: Option<f32>) -> PrimitiveArgument {
        PrimitiveArgument {
            argument_type: PrimitiveArgumentType::Float,
            flags: Some(PrimitiveArgumentFlags::Float(FloatArgumentFlags {
                min,
                max,
            })),
        }
    }
}

#[derive(Clone, Debug, PartialEq, NetEncode)]
pub enum PrimitiveArgumentFlags {
    Float(FloatArgumentFlags),
    Int(IntArgumentFlags),
    Long(LongArgumentFlags),
    String(StringArgumentType),
}

#[derive(Clone, Debug, PartialEq, Ordinalize)]
pub enum PrimitiveArgumentType {
    Bool,
    Float,
    Double,
    Int,
    Long,
    String,
    Entity,
    GameProfile,
    BlockPos,
    ColumnPos,
    Vec3,
    Vec2,
    BlockState,
    BlockPredicate,
    ItemStack,
    ItemPredicate,
    Color,
    Component,
    Style,
    Message,
    Nbt,
    NbtTag,
    NbtPath,
    Objective,
    ObjectiveCriteria,
    Operator,
    Particle,
    Angle,
    Rotation,
    ScoreboardDisplaySlot,
    ScoreHolder,
    UpTo3Axes,
    Team,
    ItemSlot,
    ResourceLocation,
    Function,
    EntityAnchor,
    IntRange,
    FloatRange,
    Dimension,
    GameMode,
    Time,
    ResourceOrTag,
    ResourceOrTagKey,
    Resource,
    ResourceKey,
    TemplateMirror,
    TemplateRotation,
    Heightmap,
    UUID,
}

impl NetEncode for PrimitiveArgumentType {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        VarInt::new(self.ordinal() as i32).encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        VarInt::new(self.ordinal() as i32)
            .encode_async(writer, opts)
            .await
    }
}

#[doc(hidden)]
mod utils {
    //! Internal utilities related to command arguments.

    /// Macro that creates a wrapper struct around an inner type and implements Deref for the inner type.
    #[macro_export]
    macro_rules! wrapper {
        (
            $(
                $(#[$meta:meta])*
                struct $name:ident $(<$($generics:tt),*>)? ($inner:ty);
            )*
        ) => {
            $(
                $(#[$meta])*
                pub struct $name $(<$($generics),*>)? ($inner);

                impl std::ops::Deref for $name {
                    type Target = $inner;

                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }
            )*
        };
    }
}
