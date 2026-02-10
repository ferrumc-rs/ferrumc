use crate::PlacementContext;
use ferrumc_world::pos::BlockPos;
use ferrumc_world::World;

/// Macro to autogenerate the `BlockBehavior` trait and associated VTable structs.
///
/// This macro simply exists to make adding methods to blocks easier. It should NOT be used anywhere except in this file, and should also only be used once.
/// See below this macro for where to add functions.
///
/// The syntax for this macro is as follows: `fn <name>([mut]; <arguments>) [-> <return type>; <default return value>]`
/// - `name`: The name of the method
/// - `mut`: Optional, whether the function takes a mutable reference to the block or not
/// - `arguments`: Any additional arguments to the method
/// - `return type`: Optional, what the function returns
/// - `default return value`: Optional, required if return type is set, the default expression to use for blocks that do not implement this method.
macro_rules! block_behavior_trait {
    ($(fn $name:ident($($mut_meta:ident)?; $($argument:ident: $ty:ty),*) $(-> $ret:ty; $default:expr)?),* $(,)?) => {
        macro_rules! ptr_ret_ty {
            (mut) => {
                u32
            };
            () => {
                ()
            };
            (mut $ret:ty) => {
                (u32, $ret)
            };
            ($ret:ty) => {
                $ret
            };
        }

        macro_rules! lambda_ret_ty {
            (mut; $data:expr;) => {
                $data
                    .try_into()
                    .unwrap_or_else(|_| panic!("Failed to convert block data back into id"))
            };
            () => {
                ()
            };
            (mut; $data:expr; $ret:ty) => {
                (
                    $data
                        .try_into()
                        .unwrap_or_else(|_| panic!("Failed to convert block data back into id")),
                    _ret,
                )
            };
            ($ret:ty) => {
                _ret
            };
        }

        pub trait BlockBehavior:
            TryInto<u32, Error = ()> + TryFrom<u32, Error = ()> + Clone + std::fmt::Debug
        {
            $(
                fn $name(&$($mut_meta)? self, $($argument: $ty),*) $(-> $ret)? { $($default)? }
            )*
        }

        impl<T> BlockBehavior for T
        where
            T: TryInto<u32, Error = ()> + TryFrom<u32, Error = ()> + Clone + std::fmt::Debug,
        {
            $(
                #[inline(always)]
                default fn $name(&$($mut_meta)? self, $($argument: $ty),*) $(-> $ret)? { $($default)? }
            )*
        }

        pub struct BlockBehaviorTable {
            $(
                $name: fn(id: u32, $($argument: $ty),*) -> ptr_ret_ty!{$($mut_meta)?}
            ),*
        }

        impl BlockBehaviorTable {
            pub const fn from<T: BlockBehavior>() -> Self {
                Self {
                    $(
                        $name: |id, $($argument),*| {
                            let $($mut_meta)? data = T::try_from(id).unwrap_or_else(|_| panic!("Failed to convert id to data"));
                            let _ret = data.$name($($argument),*);
                            lambda_ret_ty!($($mut_meta; data;)? $($ret)?)
                        }
                    ),*
                }
            }
        }

        pub struct StateBehaviorTable {
            block: &'static BlockBehaviorTable,
            id: u32,
        }

        impl StateBehaviorTable {
            pub const fn spin_off(block: &'static BlockBehaviorTable, id: u32) -> Self {
                Self { block, id }
            }

            $(
                pub fn $name(&self, $($argument: $ty),*) -> ptr_ret_ty!{$($mut_meta)? $($ret)?} {
                    (self.block.$name)(self.id, $($argument),*)
                }
            )*
        }
    };
}

/// This is where methods are defined for blocks. See the macro above for the syntax.
///
/// This is the only place where the `block_behavior_trait!` macro should be used.
block_behavior_trait!(
    fn get_placement_state(mut; _context: PlacementContext, _world: &World, _pos: BlockPos),
    fn update(mut; _world: &World, _pos: BlockPos),
    fn test(;),
);
