mod behavior;
mod slabs;
mod vtable;
mod state;
mod registry;

pub use registry::BLOCK_BEHAVIOR_REGISTRY;
pub use behavior::PlacementContext;

pub struct BlockUpdates;

pub trait BlockType {
    fn apply_changes(self) where Self: Sized;
}

#[macro_export]
macro_rules! define_block_behavior {
    ($name:ident, $({$field:ident, $property_name:ident}: $ty:ty),* $(,)?) => {
        pub struct $name<'block_data> {
            __data: &'block_data mut $crate::vanilla_chunk_format::BlockData,
            $(
                pub $field: $ty
            ),*
        }

        impl<'block_data> core::ops::Deref for $name<'block_data> {
            type Target = $crate::vanilla_chunk_format::BlockData;

            fn deref(&self) -> &Self::Target {
                &self.__data
            }
        }

        impl<'data> $name<'data> {
            const VTABLE: $crate::block::vtable::BlockBehaviorVTable = $crate::block::vtable::BlockBehaviorVTable {
                get_placement_state: |data, context, pos| $crate::block::vtable::get_placement_state_adapter::<$name>(data, context, pos),
                random_tick: |data, world, updates, pos| $crate::block::vtable::random_tick_adapter::<$name>(data, world, updates, pos),
            };

            pub fn vtable() -> &'static $crate::block::vtable::BlockBehaviorVTable {
                &$name::VTABLE
            }
        }

        impl $crate::block::BlockType for $name<'_> {
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

        impl<'block_data> std::convert::TryFrom<&'block_data mut $crate::vanilla_chunk_format::BlockData> for $name<'block_data> {
            type Error = String;

            fn try_from(data: &'block_data mut $crate::vanilla_chunk_format::BlockData) -> Result<Self, Self::Error> {
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