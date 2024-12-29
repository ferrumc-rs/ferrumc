//! TODO:
//! * Entity
//! * Double (does rust even have a double type?)
//! * Score Holder
//! * Time
//! * Resource or Tag
//! * Resource or Tag Key
//! * Resource
//! * Resource Key

use std::io::Write;

use enum_ordinalize::Ordinalize;
use ferrumc_macros::NetEncode;
use ferrumc_net_codec::{
    encode::{NetEncode, NetEncodeOpts, NetEncodeResult},
    net_types::var_int::VarInt,
};
use float::FloatParserFlags;
use int::IntParserFlags;
use long::LongParserFlags;
use string::StringParsingBehavior;
use tokio::io::AsyncWrite;

pub mod float;
pub mod int;
pub mod long;
pub mod string;

#[derive(Clone, Debug, PartialEq)]
pub struct MinecraftArgument {
    pub argument_type: MinecraftArgumentType,
    pub props: MinecraftArgumentProperties,
}

#[derive(Clone, Debug, PartialEq, NetEncode)]
pub enum MinecraftArgumentProperties {
    Float(FloatParserFlags),
    Int(IntParserFlags),
    Long(LongParserFlags),
    String(StringParsingBehavior),
}

#[derive(Clone, Debug, PartialEq, Ordinalize)]
pub enum MinecraftArgumentType {
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

impl NetEncode for MinecraftArgumentType {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()> {
        VarInt::new(self.ordinal() as i32).encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> NetEncodeResult<()> {
        VarInt::new(self.ordinal() as i32)
            .encode_async(writer, opts)
            .await
    }
}
