//! Primitive command argument types.

// TODO:
// * Entity
// * Score Holder
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

pub mod bool;
pub mod char;
pub mod float;
pub mod int;
pub mod long;
pub mod string;

/// Primitiv Argument Types that are possible for commands
#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveArgument {
    /// What [type](PrimitiveArgumentType) of of argument it is.
    pub argument_type: PrimitiveArgumentType,
    /// Which [flags](PrimitiveArgumentFlags) there are used for the type.
    pub flags: Option<PrimitiveArgumentFlags>,
}

impl PrimitiveArgument {
    /// Takes a string as argument.
    pub fn word() -> PrimitiveArgument {
        PrimitiveArgument {
            argument_type: PrimitiveArgumentType::String,
            flags: Some(PrimitiveArgumentFlags::String(StringArgumentType::Word)),
        }
    }

    /// Takes a Quotable string as argument.
    pub fn quotable() -> PrimitiveArgument {
        PrimitiveArgument {
            argument_type: PrimitiveArgumentType::String,
            flags: Some(PrimitiveArgumentFlags::String(StringArgumentType::Quotable)),
        }
    }

    /// Takes a Greedy string as argument.
    pub fn greedy() -> PrimitiveArgument {
        PrimitiveArgument {
            argument_type: PrimitiveArgumentType::String,
            flags: Some(PrimitiveArgumentFlags::String(StringArgumentType::Greedy)),
        }
    }

    /// Takes a integer as argument.
    pub fn int(min: Option<i32>, max: Option<i32>) -> PrimitiveArgument {
        PrimitiveArgument {
            argument_type: PrimitiveArgumentType::Int,
            flags: Some(PrimitiveArgumentFlags::Int(IntArgumentFlags { min, max })),
        }
    }

    /// Takes a long as argument.
    pub fn long(min: Option<i64>, max: Option<i64>) -> PrimitiveArgument {
        PrimitiveArgument {
            argument_type: PrimitiveArgumentType::Long,
            flags: Some(PrimitiveArgumentFlags::Long(LongArgumentFlags { min, max })),
        }
    }

    /// Takes a float as argument.
    pub fn float(min: Option<f32>, max: Option<f32>) -> PrimitiveArgument {
        PrimitiveArgument {
            argument_type: PrimitiveArgumentType::Float,
            flags: Some(PrimitiveArgumentFlags::Float(FloatArgumentFlags {
                min,
                max,
            })),
        }
    }

    /// Takes a boolean as argument.
    pub fn bool() -> PrimitiveArgument {
        PrimitiveArgument {
            argument_type: PrimitiveArgumentType::Bool,
            flags: None,
        }
    }
}

/// Flag types that could be given for an argument.
#[derive(Clone, Debug, PartialEq, NetEncode)]
pub enum PrimitiveArgumentFlags {
    /// Float flag
    Float(FloatArgumentFlags),
    /// Integer flag
    Int(IntArgumentFlags),
    /// Long flag
    Long(LongArgumentFlags),
    /// String flag
    String(StringArgumentType),
}

/// Types of primitive arguments that can be used in commands.
#[derive(Clone, Debug, PartialEq, Ordinalize)]
pub enum PrimitiveArgumentType {
    /// A boolean argument (`true` or `false`).
    Bool,
    /// A 32-bit floating-point number.
    Float,
    /// A 64-bit floating-point number.
    Double,
    /// A 32-bit integer argument.
    Int,
    /// A 64-bit integer argument.
    Long,
    /// A string argument.
    String,
    /// A single or multiple entity selector.
    Entity,
    /// A Minecraft game profile (player name + UUID).
    GameProfile,
    /// A block position (`x, y, z`).
    BlockPos,
    /// A 2D column position (`x, z`).
    ColumnPos,
    /// A 3D vector of floats.
    Vec3,
    /// A 2D vector of floats.
    Vec2,
    /// A block state argument.
    BlockState,
    /// A predicate that matches blocks.
    BlockPredicate,
    /// An item stack argument.
    ItemStack,
    /// A predicate that matches items.
    ItemPredicate,
    /// A color argument.
    Color,
    /// A text component argument.
    Component,
    /// A text style argument.
    Style,
    /// A chat message argument.
    Message,
    /// An NBT data argument.
    Nbt,
    /// A single NBT tag.
    NbtTag,
    /// An NBT path for selecting nested tags.
    NbtPath,
    /// A scoreboard objective.
    Objective,
    /// Criteria for a scoreboard objective.
    ObjectiveCriteria,
    /// A mathematical operator (`+`, `-`, etc.).
    Operator,
    /// A particle type argument.
    Particle,
    /// An angle in degrees.
    Angle,
    /// A 3D rotation argument.
    Rotation,
    /// A scoreboard display slot.
    ScoreboardDisplaySlot,
    /// A player or entity score holder.
    ScoreHolder,
    /// Up to 3 axes for relative movement.
    UpTo3Axes,
    /// A team name argument.
    Team,
    /// An item slot argument.
    ItemSlot,
    /// A namespaced resource identifier.
    ResourceLocation,
    /// A function identifier.
    Function,
    /// An anchor point on an entity.
    EntityAnchor,
    /// A range of integers.
    IntRange,
    /// A range of floating-point numbers.
    FloatRange,
    /// A dimension identifier.
    Dimension,
    /// A game mode argument.
    GameMode,
    /// A time value argument.
    Time,
    /// A resource or tag identifier.
    ResourceOrTag,
    /// A key for a resource or tag.
    ResourceOrTagKey,
    /// A resource identifier.
    Resource,
    /// A key to a resource.
    ResourceKey,
    /// Template mirror argument (e.g., NONE, LEFT_RIGHT, FRONT_BACK).
    TemplateMirror,
    /// Template rotation argument.
    TemplateRotation,
    /// A heightmap argument.
    Heightmap,
    /// A UUID argument.
    UUID,
}

impl NetEncode for PrimitiveArgumentType {
    /// Encodes the type for networking.
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        VarInt::new(self.ordinal() as i32).encode(writer, opts)
    }

    /// ENcodes the type for networing in async.
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
