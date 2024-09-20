use crate::NbtToken;

pub trait NbtOwnedValue {
    fn from_token(token: NbtToken) -> Self;
}


// for basic types, like i32, i64, f32, f64, bool => no need to copy, since, they're already owned, 
// since a pointer is 64 bits, and the data itself is smaller :)

macro_rules! impl_no_clone {
    ($($($ty:ty)|* > $variant:ident),*) => {
        $(
            impl NbtOwnedValue for $ty {
                fn from_token(token: NbtToken) -> Self {
                    match token {
                        NbtToken::$variant(val) => val as $ty,
                        _ => panic!("Expected {:?}, got {:?}", stringify!($variant), token)
                    }
                }
            }
        )*
    };
}

impl_no_clone!(
    i8 | u8 > Byte ,
    i16 | u16 > Short,
    i32 | u32 > Int,
    i64 | u64 > Long,
    f32 > Float,
    f64 > Double
);


impl NbtOwnedValue for bool {
    fn from_token(token: NbtToken) -> Self {
        match token {
            NbtToken::Byte(val) => val != 0,
            _ => panic!("Expected Byte, got {:?}", token)
        }
    }
}

impl NbtOwnedValue for String {
    fn from_token(token: NbtToken) -> Self {
        match token {
            NbtToken::String(val) => val.to_string(),
            _ => panic!("Expected String, got {:?}", token)
        }
    }
}

