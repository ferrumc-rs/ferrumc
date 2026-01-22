use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;
use lazy_static::lazy_static;
use tracing::error;
use crate::pos::BlockPos;
use crate::vanilla_chunk_format::BlockData;
use crate::World;

pub struct BlockUpdates;

pub struct BlockBehaviorVTable {
    pub random_tick: fn(
        data: &mut BlockData,
        world: &World,
        updates: &mut BlockUpdates,
        pos: BlockPos,
    ),
    pub test: fn(),
}

pub trait BlockType {
    fn apply_changes(self) where Self: Sized;
}

#[inline(always)]
fn random_tick_adapter<'data, T: BlockBehavior + BlockType + TryFrom<&'data mut BlockData, Error=String> + 'data>(
    data: &'data mut BlockData,
    world: &World,
    updates: &mut BlockUpdates,
    pos: BlockPos,
) {
    match T::try_from(data) {
        Ok(mut behavior) => {
            behavior.random_tick(world, updates, pos);
            behavior.apply_changes();
        },
        Err(msg) => error!("Failed to convert BlockData into block behavior: {msg}"),
    }
}

#[inline(always)]
fn test_adapter<T: Any>() {
    println!("{:?}", TypeId::of::<T>());
}

macro_rules! define_block_behavior {
    ($name:ident, $({$field:ident, $property_name:ident}: $ty:ty),* $(,)?) => {
        pub struct $name<'block_data> {
            __data: &'block_data mut BlockData,
            $(
                pub $field: $ty
            ),*
        }

        impl<'block_data> Deref for $name<'block_data> {
            type Target = BlockData;

            fn deref(&self) -> &Self::Target {
                &self.__data
            }
        }

        impl<'data> $name<'data> {
            const VTABLE: BlockBehaviorVTable = BlockBehaviorVTable {
                random_tick: |data, world, updates, pos| random_tick_adapter::<$name>(data, world, updates, pos),
                test: test_adapter::<$name>,
            };

            pub fn vtable() -> &'static BlockBehaviorVTable {
                &$name::VTABLE
            }
        }

        impl BlockType for $name<'_> {
            fn apply_changes(self) {
                $(
                    self
                        .__data
                        .properties
                        .as_mut()
                        .expect("should exist")
                        .insert(stringify!($property_name).into(), self.$field.to_string());
                )*
            }
        }

        impl<'block_data> TryFrom<&'block_data mut BlockData> for $name<'block_data> {
            type Error = String;

            fn try_from(data: &'block_data mut BlockData) -> Result<Self, Self::Error> {
                match &data.properties {
                    Some(properties) => {
                        Ok(Self {
                            $(
                                $field: properties
                                    .get(&stringify!($property_name).to_string())
                                    .ok_or_else(||
                                        format!("Field {} on block {} does not exist in the BlockData properties", stringify!($field), stringify!($name))
                                    )?
                                    .parse::<$ty>()
                                    .map_err(|_|
                                        format!("Failed to parse field {} on block {} from property string", stringify!($field), stringify!($name))
                                    )?
                            ),*,
                            __data: data,
                        })
                    },
                    None => Err("BlockData does not contain a property map".to_string())
                }
            }
        }
    };
}

pub trait BlockBehavior {
    fn random_tick(&mut self, _world: &World, _updates: &mut BlockUpdates, _pos: BlockPos) {}
}

define_block_behavior!(CropBlock, {age, age}: u8);
define_block_behavior!(SlabBlock, {slab_type, type}: SlabType, {waterlogged, waterlogged}: bool);

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

impl BlockBehavior for CropBlock<'_> {
    fn random_tick(&mut self, _world: &World, _updates: &mut BlockUpdates, _pos: BlockPos) {
        if self.age < 7 {
            self.age += 1;
        }
    }
}

impl BlockBehavior for SlabBlock<'_> {}

lazy_static! {
    pub static ref BLOCK_BEHAVIOR_REGISTRY: HashMap<&'static str, &'static BlockBehaviorVTable> = {
        let mut m = HashMap::new();

        m.insert("minecraft:wheat", CropBlock::vtable());
        m.insert("minecraft:carrot", CropBlock::vtable());
        m.insert("minecraft:potato", CropBlock::vtable());

        m
    };
}