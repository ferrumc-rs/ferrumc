use std::str::FromStr;

mod slab_type;

/// Marker trait for types that can be used as block state property values
pub trait BlockStateProperty: FromStr + ToString + Default {}

impl BlockStateProperty for u8 {}
impl BlockStateProperty for u16 {}
impl BlockStateProperty for u32 {}
impl BlockStateProperty for u64 {}

impl BlockStateProperty for i8 {}
impl BlockStateProperty for i16 {}
impl BlockStateProperty for i32 {}
impl BlockStateProperty for i64 {}

impl BlockStateProperty for f32 {}
impl BlockStateProperty for f64 {}

impl BlockStateProperty for String {}
impl BlockStateProperty for bool {}